#pragma once

#include "common.h"
#include "value.h"
typedef enum {
    OP_CONSTANT,
    OP_RETURN,
    OP_NEGATIVE,  // -
    OP_NIL,
    OP_TRUE,
    OP_FALSE,
    OP_NOT,       // logical not
    OP_ADD,       // +
    OP_MULTIPLY,  // *
    OP_DIVIDE,    // /
    OP_SUBTRACT,  // -
    OP_EQUAL,
    OP_GREATER,
    OP_LESS,
} OpCode;

typedef struct {
    int        count;
    int        capacity;
    uint8_t*   code;
    int*       lines;
    ValueArray constants;
} Chunk;

void init_chunk(Chunk* chunk);

void write_chunk(Chunk* chunk, uint8_t byte, int line);
/**
 * @brief 添加常量到块中
 *
 * 将给定的常量值添加到给定的块中，并返回常量在块中的索引位置（从0开始）。
 *
 * @param chunk 块指针
 * @param value 常量值
 *
 * @return 返回常量在块中的索引位置（从0开始）
 */
int add_constant(Chunk* chunk, Value value);

void free_chunk(Chunk* chunk);