#include "compiler.h"
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include "chunk.h"
#include "debug.h"
#include "object.h"
#include "scanner.h"
#include "value.h"
#ifdef DEBUG_PRINT_CODE
#include "debug.h"
#endif

typedef struct {
    Token current;
    Token previous;
    bool  has_error;
    bool  panic_mode;
} Parser;
// 优先级序列
typedef enum {
    PREC_NONE,
    PREC_ASSIGNMENT,  // =
    PREC_OR,          // or
    PREC_AND,         // and
    PREC_EQUALITY,    // == !=
    PREC_COMPARISON,  // < > <= >=
    PREC_TERM,        // + -
    PREC_FACTOR,      // * /
    PREC_UNARY,       // ! -
    PREC_CALL,        // . ()
    PREC_PRIMARY
} Precedence;

typedef void (*ParseFn)();

typedef struct {
    ParseFn    prefix;
    ParseFn    infix;
    Precedence precedence;
} ParseRule;

Parser parser;
Chunk* compilingChunk;

static void       expression();
static ParseRule* getRule(TokenType type);
static void       parse_precedence(Precedence precedence);

static void statement();
static void expression_statement();
static void declaration();

static void       error_at(Token* token, const char* msg)

{
    if (parser.panic_mode)
        return;
    parser.panic_mode = true;
    fprintf(stderr, "[line %d] Error", token->line);

    if (token->type == TOKEN_EOF) {
        fprintf(stderr, " at end");
    } else if (token->type == TOKEN_ERROR) {
        // Nothing.
    } else {
        fprintf(stderr, " at '%.*s'", token->length, token->start);
    }

    fprintf(stderr, ": %s\n", msg);
    parser.has_error = true;
}

static void error(const char* message)
{
    error_at(&parser.previous, message);
}
static void error_at_current(const char* msg)
{
    error_at(&parser.current, msg);
}
static void advance()
{
    parser.previous = parser.current;
    for (;;) {
        parser.current = scan_token();
        if (parser.current.type != TOKEN_ERROR) {
            break;
        }
        error_at_current(parser.current.start);
    }
}

static void reset()
{
    parser.panic_mode = false;
    parser.has_error = false;
}

static void consume(TokenType type, const char* msg)
{
    if (parser.current.type == type) {
        advance();
        return;
    }
    error_at_current(msg);
}

static Chunk* current_chunk()
{
    return compilingChunk;
}

static void emit_byte(uint8_t byte)
{
    write_chunk(current_chunk(), byte, parser.previous.line);
}

static void emit_bytes(uint8_t byte1, uint8_t byte2)
{
    emit_byte(byte1);
    emit_byte(byte2);
}

static void emit_return()
{
    emit_byte(OP_RETURN);
}
static void end_compiler()
{
    emit_return();
#ifdef DEBUG_PRINT_CODE
    if (!parser.has_error) {
        disassemble_chunk(current_chunk(), "code");
    }
#endif
}

static uint8_t make_constant(Value val)
{
    int constant = add_constant(current_chunk(), val);
    if (constant > UINT8_MAX) {
        error("too many constant in one chunk\n");
        return 0;
    }
    return (uint8_t)constant;
}

static void emit_constant(Value val)
{
    emit_bytes(OP_CONSTANT, make_constant(val));
}

static void parse_precedence(Precedence precedence)
{
    advance();
    ParseFn prefixRule = getRule(parser.previous.type)->prefix;
    if (prefixRule == NULL) {
        error("Expect expression.");
        return;
    }

    prefixRule();

    while (precedence <= getRule(parser.current.type)->precedence) {
        advance();
        ParseFn infixRule = getRule(parser.previous.type)->infix;
        infixRule();
    }
}

static void number()
{
    double val = strtod(parser.previous.start, NULL);
    emit_constant(NUMBER_VAL(val));
}

static void string()
{
    // -2 是把两个引号去掉
    emit_constant(OBJ_VAL(copy_string1(parser.previous.start + 1,
                                       parser.previous.length - 2)));
};

static void expression()
{
    parse_precedence(PREC_ASSIGNMENT);
}

static void grouping()
{
    expression();
    consume(TOKEN_RIGHT_PAREN, "expect ')' after expression");
}

static void unary()
{
    TokenType operator= parser.previous.type;
    parse_precedence(PREC_UNARY);
    switch (operator) {
    case TOKEN_MINUS:
        emit_byte(OP_NEGATIVE);
        break;
    case TOKEN_BANG:
        emit_byte(OP_NOT);
    default:
        return;
    }
}

static void literal()
{
    switch (parser.previous.type) {
    case TOKEN_FALSE:
        emit_byte(OP_FALSE);
        break;
    case TOKEN_NIL:
        emit_byte(OP_NIL);
        break;
    case TOKEN_TRUE:
        emit_byte(OP_TRUE);
        break;
    default:
        return;  // Unreachable.
    }
}

static bool check(TokenType type)
{
    return parser.current.type == type;
}

static bool match(TokenType type)
{
    if (!check(type))
        return false;
    advance();
    return true;
}

static void binary()
{
    TokenType  operatorType = parser.previous.type;
    ParseRule* rule = getRule(operatorType);
    parse_precedence((Precedence)(rule->precedence + 1));

    switch (operatorType) {
    case TOKEN_BANG_EQUAL:
        emit_bytes(OP_EQUAL, OP_NOT);
        break;
    case TOKEN_EQUAL_EQUAL:
        emit_byte(OP_EQUAL);
        break;
    case TOKEN_GREATER:
        emit_byte(OP_GREATER);
        break;
    case TOKEN_GREATER_EQUAL:
        emit_bytes(OP_LESS, OP_NOT);
        break;
    case TOKEN_LESS:
        emit_byte(OP_LESS);
        break;
    case TOKEN_LESS_EQUAL:
        emit_bytes(OP_GREATER, OP_NOT);
        break;
    case TOKEN_PLUS:
        emit_byte(OP_ADD);
        break;
    case TOKEN_MINUS:
        emit_byte(OP_SUBTRACT);
        break;
    case TOKEN_STAR:
        emit_byte(OP_MULTIPLY);
        break;
    case TOKEN_SLASH:
        emit_byte(OP_DIVIDE);
        break;
    default:
        return;  // Unreachable.
    }
}

static void print_statement()
{
    expression();
    consume(TOKEN_SEMICOLON, "expect ; after print statement");
    emit_byte(OP_PRINT);
}

static void statement()
{
    if (match(TOKEN_PRINT)) {
        print_statement();
    } else {
        expression_statement();
    }
}

static void sync()
{
    parser.panic_mode = false;

    while (parser.current.type != TOKEN_EOF) {
        if (parser.previous.type == TOKEN_SEMICOLON)
            return;
        switch (parser.current.type) {
        case TOKEN_CLASS:
        case TOKEN_FUN:
        case TOKEN_VAR:
        case TOKEN_FOR:
        case TOKEN_IF:
        case TOKEN_WHILE:
        case TOKEN_PRINT:
        case TOKEN_RETURN:
            return;

        default:;  // Do nothing.
        }

        advance();
    }
}

static void var_declaration() {}

static void declaration()
{
    // if (match(TOKEN_VAR)) {
    //     var_declaration();
    // } else {
    //     statement();
    // }
    statement();

    if (parser.panic_mode) {
        sync();
    }
}

static void expression_statement()
{
    expression();
    consume(TOKEN_SEMICOLON, "expect ; after expression statement");
    emit_byte(OP_POP);
}

ParseRule rules[] = {
    [TOKEN_LEFT_PAREN] = {grouping, NULL, PREC_NONE},
    [TOKEN_RIGHT_PAREN] = {NULL, NULL, PREC_NONE},
    [TOKEN_LEFT_BRACE] = {NULL, NULL, PREC_NONE},
    [TOKEN_RIGHT_BRACE] = {NULL, NULL, PREC_NONE},
    [TOKEN_COMMA] = {NULL, NULL, PREC_NONE},
    [TOKEN_DOT] = {NULL, NULL, PREC_NONE},
    [TOKEN_MINUS] = {unary, binary, PREC_TERM},
    [TOKEN_PLUS] = {NULL, binary, PREC_TERM},
    [TOKEN_SEMICOLON] = {NULL, NULL, PREC_NONE},
    [TOKEN_SLASH] = {NULL, binary, PREC_FACTOR},
    [TOKEN_STAR] = {NULL, binary, PREC_FACTOR},
    [TOKEN_BANG] = {unary, NULL, PREC_NONE},
    [TOKEN_BANG_EQUAL] = {NULL, binary, PREC_EQUALITY},
    [TOKEN_EQUAL] = {NULL, NULL, PREC_NONE},
    [TOKEN_EQUAL_EQUAL] = {NULL, binary, PREC_EQUALITY},
    [TOKEN_GREATER] = {NULL, binary, PREC_COMPARISON},
    [TOKEN_GREATER_EQUAL] = {NULL, binary, PREC_COMPARISON},
    [TOKEN_LESS] = {NULL, binary, PREC_COMPARISON},
    [TOKEN_LESS_EQUAL] = {NULL, binary, PREC_COMPARISON},
    [TOKEN_IDENTIFIER] = {NULL, NULL, PREC_NONE},
    [TOKEN_STRING] = {string, NULL, PREC_NONE},
    [TOKEN_NUMBER] = {number, NULL, PREC_NONE},
    [TOKEN_AND] = {NULL, NULL, PREC_NONE},
    [TOKEN_CLASS] = {NULL, NULL, PREC_NONE},
    [TOKEN_ELSE] = {NULL, NULL, PREC_NONE},
    [TOKEN_FALSE] = {literal, NULL, PREC_NONE},
    [TOKEN_FOR] = {NULL, NULL, PREC_NONE},
    [TOKEN_FUN] = {NULL, NULL, PREC_NONE},
    [TOKEN_IF] = {NULL, NULL, PREC_NONE},
    [TOKEN_NIL] = {literal, NULL, PREC_NONE},
    [TOKEN_OR] = {NULL, NULL, PREC_NONE},
    [TOKEN_PRINT] = {NULL, NULL, PREC_NONE},
    [TOKEN_RETURN] = {NULL, NULL, PREC_NONE},
    [TOKEN_SUPER] = {NULL, NULL, PREC_NONE},
    [TOKEN_THIS] = {NULL, NULL, PREC_NONE},
    [TOKEN_TRUE] = {literal, NULL, PREC_NONE},
    [TOKEN_VAR] = {NULL, NULL, PREC_NONE},
    [TOKEN_WHILE] = {NULL, NULL, PREC_NONE},
    [TOKEN_ERROR] = {NULL, NULL, PREC_NONE},
    [TOKEN_EOF] = {NULL, NULL, PREC_NONE},
};

static ParseRule* getRule(TokenType type)
{
    return &rules[type];
}

bool compile(const char* source, Chunk* chunk)
{
    init_scanner(source);
    // PARSER
    compilingChunk = chunk;
    reset();
    advance();
    // expression();
    // consume(TOKEN_EOF, "expect end of line\n");
    while (!match(TOKEN_EOF)) {
        declaration();
    }
    end_compiler();
    return !parser.has_error;
    // END PARSER
    // int line = -1;
    // for (;;) {
    //     Token token = scan_token();
    //     if (token.line != line) {
    //         printf("%4d ", token.line);
    //         line = token.line;
    //     } else {
    //         printf("   | ");
    //     }
    //     printf("%2d '%.*s' %s\n", token.type, token.length, token.start,
    //            tokentype_2_string(token.type));

    //     if (token.type == TOKEN_EOF)
    //         break;
    // }
}