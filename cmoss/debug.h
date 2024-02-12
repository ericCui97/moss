#ifndef CMOSS_DEBUG_H
#define CMOSS_DEBUG_H
#include "chunk.h"

void disassembleChunk(Chunk *chunk, const char *name);
int disassembleInstruction(Chunk *chunk, int offset);
#endif
//> Chunks of Bytecode debug-h
#ifndef cmoss_debug_h
#define cmoss_debug_h

#include "chunk.h"

void disassembleChunk(Chunk* chunk, const char* name);
int disassembleInstruction(Chunk* chunk, int offset);

#endif
