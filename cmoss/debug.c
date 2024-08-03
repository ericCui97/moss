#include "debug.h"
#include <stdint.h>
#include <stdio.h>
#include "chunk.h"
#include "value.h"

void disassemble_chunk(Chunk* chunk, const char* name)
{
    printf("==%s==\n", name);

    for (int offset = 0; offset < chunk->count;) {
        offset = disassemble_instruction(chunk, offset);
    }
}

static int simple_instruction(const char* name, int offset)
{
    printf("%s\n", name);
    return offset + 1;
}

static int constant_instruction(const char* name, Chunk* chunk, int offset)
{
    uint8_t constant = chunk->code[offset + 1];
    printf("%-16s %4d '", name, constant);
    print_value(chunk->constants.values[constant]);
    printf("'\n");
    return offset + 2;
}

int disassemble_instruction(Chunk* chunk, int offset)
{
    printf("%04d ", offset);

    if (offset > 0 && chunk->lines[offset] == chunk->lines[offset - 1]) {
        printf("   | ");
    } else {
        printf("%4d ", chunk->lines[offset]);
    }
    uint8_t instruction = chunk->code[offset];

    switch (instruction) {
    case OP_NEGATIVE:
        return simple_instruction("OP_NEGATIVE", offset);
    case OP_ADD:
        return simple_instruction("add", offset);
    case OP_SUBTRACT:
        return simple_instruction("sub", offset);
    case OP_MULTIPLY:
        return simple_instruction("mul", offset);
    case OP_DIVIDE:
        return simple_instruction("div", offset);
    case OP_CONSTANT:
        return constant_instruction("OP_CONSTANT", chunk, offset);
    case OP_RETURN:
        return simple_instruction("OP_RETURN", offset);
    case OP_NIL:
        return simple_instruction("OP_NIL", offset);
    case OP_TRUE:
        return simple_instruction("OP_TRUE", offset);
    case OP_FALSE:
        return simple_instruction("OP_FALSE", offset);
    case OP_NOT:
        return simple_instruction("OP_NOT", offset);
    case OP_EQUAL:
        return simple_instruction("OP_EQUAL", offset);
    case OP_GREATER:
        return simple_instruction("OP_GREATER", offset);
    case OP_LESS:
        return simple_instruction("OP_LESS", offset);
    case OP_PRINT:
        return simple_instruction("OP_PRINT", offset);
    case OP_POP:

        return simple_instruction("OP_POP", offset);

    case OP_DEFINE_GLOBAL:
        return constant_instruction("OP_DEFINE_GLOBAL", chunk, offset);

    case OP_GET_GLOBAL:
        return constant_instruction("OP_GET_GLOBAL", chunk, offset);
    case OP_SET_GLOBAL:
        return constant_instruction("OP_SET_GLOBAL", chunk, offset);
    default:
        printf("unknown op code %d\n", instruction);
        return offset + 1;
    }
}
