#pragma once
#include <stdint.h>
#include "chunk.h"
#include "value.h"
#define STACK_MAX 256
#define READ_CONSTANT() (vm.chunk->constants.values[READ_BYTE()])

typedef struct {
    Chunk*   chunk;
    uint8_t* ip;
    Value    stack[STACK_MAX];
    Value*   stackTop;
} VM;

typedef enum {
    INTERPRET_OK,
    INTERPRET_COMPILE_ERROR,
    INTERPRET_RUNTIME_ERROR
} InterpretResult;

void            initVM();
void            freeVM();
InterpretResult interpret(Chunk* chunk);

void  push(Value value);
Value pop();