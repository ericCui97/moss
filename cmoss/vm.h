#pragma once
#include <stdint.h>
#include "chunk.h"
#include "common.h"
#include "hashtable.h"
#include "value.h"
#define STACK_MAX 256
typedef struct {
    Chunk* chunk;
    uint8_t* ip; // instruction pointer 也就是pc指针
    Value stack[STACK_MAX];
    Value*   stackTop;
    Table    strings;
    Obj*     objects;
    Table    globals;

} VM;

extern VM vm;

typedef enum{
 INTERPRET_OK,
  INTERPRET_COMPILE_ERROR,
  INTERPRET_RUNTIME_ERROR
} InterpretResult;

void init_vm();
void free_vm();

void push(Value value);
Value pop();

InterpretResult interpret(char* source);