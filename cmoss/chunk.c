#include "chunk.h"
#include <stdint.h>
#include <stdlib.h>
#include "memory.h"
#include "value.h"
void init_chunk(Chunk* chunk)
{
    chunk->capacity = 0;
    chunk->count = 0;
    chunk->code = NULL;
    chunk->lines = NULL;
    init_value_array(&chunk->constants);
}

void write_chunk(Chunk* chunk, uint8_t byte, int line)
{
    if (chunk->capacity < chunk->count + 1) {
        // 扩容
        int oldCapacity = chunk->capacity;
        chunk->capacity = GROW_CAPACITY(oldCapacity);
        chunk->code =
            GROW_ARRAY(uint8_t, chunk->code, oldCapacity, chunk->capacity);
        chunk->lines = GROW_ARRAY(int, chunk->lines, oldCapacity, chunk->capacity);
    }

    chunk->code[chunk->count] = byte;
    chunk->lines[chunk->count] = line;
    chunk->count++;
}

void free_chunk(Chunk* chunk)
{
    FREE_ARRAY(uint8_t, chunk->code, chunk->capacity);
    FREE_ARRAY(int, chunk->lines, chunk->capacity);
    free_value_array(&chunk->constants);
    init_chunk(chunk);
}

int add_constant(Chunk* chunk, Value value)
{
    write_value_array(&chunk->constants, value);
    return chunk->constants.count - 1;
}