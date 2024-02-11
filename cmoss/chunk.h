#ifndef clox_chunk_h
#define clox_chunk_h

#include <stdint.h>
#include "common.h"
#include "value.h"
typedef enum {
    OP_RETURN,
    OP_NEGATE,
    OP_CONSTANT,

} OpCode;

typedef struct {
    int        count;
    int        capacity;
    uint8_t*   code;
    int*       lines;
    ValueArray constants;

} Chunk;
// init chunk
void initChunk(Chunk* chunk);

// write a byte into the chunk
void writeChunk(Chunk* chunk, uint8_t byte, int line);

//  free the chunk
void freeChunk(Chunk* chunk);

int addConstant(Chunk* chunk, Value value);
#endif