#include "chunk.h"
#include "common.h"
#include "memory.h"

int main(int argc,char** argv){
    Chunk chunk;
    initChunk(&chunk);
    writeChunk(&chunk,OP_RETURN);
    freeChunk(&chunk);
    printf("Hello, World!\n");
  return 0;
}