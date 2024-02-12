#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include "chunk.h"
#include "common.h"
#include "debug.h"
#include "memory.h"
#include "vm.h"

static char* readFile(const char* path) {
  FILE* file = fopen(path, "rb");

   if (file == NULL) {
    fprintf(stderr, "Could not open file \"%s\".\n", path);
    exit(74);
  }

  fseek(file, 0L, SEEK_END);
  size_t fileSize = ftell(file);
  rewind(file);

  char* buffer = (char*)malloc(fileSize + 1);
  size_t bytesRead = fread(buffer, sizeof(char), fileSize, file);

  if (bytesRead < fileSize) {
    fprintf(stderr, "Could not read file \"%s\".\n", path);
    exit(74);
  }
  buffer[bytesRead] = '\0';

  fclose(file);
  return buffer;
}


static void repl() {
  char line[1024];
  for (;;) {
    printf("> ");

    if (!fgets(line, sizeof(line), stdin)) {
      printf("\n");
      break;
    }

    interpret(line);
  }
}


static void runFile(const char* path) {
  char* source = readFile(path);
  InterpretResult result = interpret(source);
  free(source); 

  if (result == INTERPRET_COMPILE_ERROR) exit(65);
  if (result == INTERPRET_RUNTIME_ERROR) exit(70);
}


int main(int argc, char** argv)
{
    initVM();
    // int constant = addConstant(&chunk, 1.2);
    // writeChunk(&chunk, OP_CONSTANT,123);
    // writeChunk(&chunk, constant,123);
    // writeChunk(&chunk, OP_NEGATE, 123);
    // writeChunk(&chunk, OP_RETURN,123);
    // disassembleChunk(&chunk, "test chunk");
    // interpret(&chunk);
    // freeVM();
    // freeChunk(&chunk);
    // printf("Hello, World!\n");
    if (argc == 1) {
    repl();
  } else if (argc == 2) {
    runFile(argv[1]);
  } else {
    fprintf(stderr, "Usage: clox [path]\n");
    exit(64);
  }

  freeVM();

    return 0;
}