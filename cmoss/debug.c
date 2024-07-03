#include "debug.h"
#include <stdint.h>
#include <stdio.h>

void disassemble_chunk(Chunk* chunk, const char* name)
{
    printf("==%s==\n", name);

    for (int offset = 0; offset < chunk->count;) {
        offset = disassemble_instruction(chunk, offset);
    }
}


static int simple_instruction(const char* name, int offset) {
    printf("%s\n",name);
    return offset + 1;
}

int disassemble_instruction(Chunk* chunk, int offset) {
    printf("%04d ",offset);
    uint8_t instruction = chunk->code[offset];

    switch(instruction){
        case OP_RETURN:
        simple_instruction("OP_RETURN",offset);
        default:
        printf("unknown op code %d\n",instruction);
        return offset + 1; 
    }
}


