#ifndef clox_chunk_h
#define clox_chunk_h

#include "common.h"
#include <stdint.h>
typedef enum {
  OP_RETURN,
} OpCode;

typedef struct {
  int count;
  int capacity;
  uint8_t *code;
} Chunk;
// init chunk
void initChunk(Chunk *chunk);

// write a byte into the chunk
void writeChunk(Chunk *chunk,uint8_t byte);

//  free the chunk
void freeChunk(Chunk *chunk);
#endif