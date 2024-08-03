#include "vm.h"
#include <stdarg.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include "chunk.h"
#include "compiler.h"
#include "debug.h"
#include "hashtable.h"
#include "memory.h"
#include "object.h"
#include "value.h"

VM vm;

static Value peek(int distance)
{
    return vm.stackTop[-1 - distance];
}

static void reset_stack()
{
    vm.stackTop = vm.stack;
}

static void runtimeError(const char* format, ...)
{
    va_list args;
    va_start(args, format);
    vfprintf(stderr, format, args);
    va_end(args);
    fputs("\n", stderr);

    size_t instruction = vm.ip - vm.chunk->code - 1;
    int    line = vm.chunk->lines[instruction];
    fprintf(stderr, "[line %d] in script\n", line);
    reset_stack();
}

void init_vm()
{
    reset_stack();
    init_table(&vm.globals);
    vm.objects = NULL;
    init_table(&vm.strings);
}

void free_vm()
{
    free_table(&vm.globals);
    free_table(&vm.strings);
    free_objects();
}

static bool is_falsey(Value value)
{
    return IS_NIL(value) || (IS_BOOL(value) && !AS_BOOL(value));
}
static void concatenate()
{
    ObjString* b = AS_STRING(pop());
    ObjString* a = AS_STRING(pop());

    int   length = a->length + b->length;
    char* chars = ALLOCATE(char, length + 1);
    memcpy(chars, a->chars, a->length);
    memcpy(chars + a->length, b->chars, b->length);
    chars[length] = '\0';

    ObjString* result = take_string(chars, length);
    push(OBJ_VAL(result));
}

void push(Value value)
{
    *vm.stackTop = value;
    ++vm.stackTop;
}

Value pop()
{
    --vm.stackTop;
    return *vm.stackTop;
}

static InterpretResult run()
{
#define READ_BYTE() (*vm.ip++)
#define READ_CONSTANT() (vm.chunk->constants.values[READ_BYTE()])
#define READ_STRING() AS_STRING(READ_CONSTANT())

#define BINARY_OP(valueType, op)                                           \
    do {                                                                   \
        if (!IS_NUMBER(peek(0)) || !IS_NUMBER(peek(1))) {                  \
            runtimeError("Operands must be numbers.");                     \
            return INTERPRET_RUNTIME_ERROR;                                \
        }                                                                  \
        double b = AS_NUMBER(pop());                                       \
        double a = AS_NUMBER(pop());                                       \
        push(valueType(a op b));                                           \
    } while (false)

    for (;;) {
#ifdef DEBUG_TRACE_EXECUTION
        printf("==========stack==========\n");
        for (Value* slot = vm.stack; slot < vm.stackTop; slot++) {
            printf("[ ");
            print_value(*slot);
            printf(" ]");
        }
        printf("\n");
        disassemble_instruction(vm.chunk, (int)(vm.ip - vm.chunk->code));
#endif
        uint8_t instruction;
        switch (instruction = READ_BYTE()) {
        case OP_PRINT: {
            // printf("goto print");
            print_value(pop());
            printf("\n");
            break;
        }
        case OP_RETURN: {
            // print_value(pop());
            // printf("\n");
            // exit the vm;
            return INTERPRET_OK;
        }

        case OP_CONSTANT: {
            Value constant = READ_CONSTANT();
            push(constant);
            printf("\n");
            break;
        }
        case OP_ADD:
            if (IS_STRING(peek(0)) && IS_STRING(peek(1))) {
                concatenate();
            } else if (IS_NUMBER(peek(0)) && IS_NUMBER(peek(1))) {
                double b = AS_NUMBER(pop());
                double a = AS_NUMBER(pop());
                push(NUMBER_VAL(a + b));
            } else {
                runtimeError(
                    "Operands must be two numbers or two strings.");
                return INTERPRET_RUNTIME_ERROR;
            }
            break;
            break;
        case OP_SUBTRACT:
            BINARY_OP(NUMBER_VAL, -);
            break;
        case OP_MULTIPLY:
            BINARY_OP(NUMBER_VAL, *);
            break;
        case OP_DIVIDE:
            BINARY_OP(NUMBER_VAL, /);
            break;
        case OP_NEGATIVE: {
            if (!IS_NUMBER(peek(0))) {
                runtimeError("Operand must be a number.");
            }
            push(NUMBER_VAL(-AS_NUMBER(pop())));
            break;
        }
        case OP_NIL:
            push(NIL_VAL);
            break;
        case OP_TRUE:
            push(BOOL_VAL(true));
            break;
        case OP_FALSE:
            push(BOOL_VAL(false));
            break;
        case OP_NOT:
            push(BOOL_VAL(is_falsey(pop())));
        case OP_EQUAL: {
            Value b = pop();
            Value a = pop();
            push(BOOL_VAL(values_equal(a, b)));
            break;
        }
        case OP_GREATER:
            BINARY_OP(BOOL_VAL, >);
            break;
        case OP_LESS:
            BINARY_OP(BOOL_VAL, <);
            break;

        case OP_POP:
            pop();
            break;
        case OP_DEFINE_GLOBAL: {
            ObjString* name = READ_STRING();
            table_set(&vm.globals, name, peek(0));
            pop();
            break;
        }
        case OP_GET_GLOBAL: {
            ObjString* name = READ_STRING();
            Value      value;
            if (!table_get(&vm.globals, name, &value)) {
                runtimeError("undefined val %s\n", name->chars);
                return INTERPRET_RUNTIME_ERROR;
            }
            push(value);
            break;
        }
        }
    }

#undef READ_BYTE
#undef READ_CONSTANT
#undef BINARY_OP
}

// InterpretResult interpret(Chunk* chunk)
// {
//     vm.chunk = chunk;
//     vm.ip = vm.chunk->code;
//     return run();
// }

InterpretResult interpret(char* source)
{
    printf("%s\n",source);
    Chunk chunk;
    init_chunk(&chunk);
    if (!compile(source, &chunk)) {
        free_chunk(&chunk);
        return INTERPRET_COMPILE_ERROR;
    }

    vm.chunk = &chunk;
    vm.ip = vm.chunk->code;

    InterpretResult result = run();
    free_chunk(&chunk);
    return result;
}
