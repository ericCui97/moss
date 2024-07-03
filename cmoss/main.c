#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "chunk.h"
#include "common.h"
#include "debug.h"
#include "vm.h"

static void repl()
{
    char line[1024];
    for (;;) {
        printf("> ");

        if (!fgets(line, sizeof(line), stdin)) {
            printf("\n");
            break;  // EOF
        }
        interpret(line);
        memset(line, 0, sizeof(line));
    }
}

static char* readFile(const char* path)
{
    FILE* file = fopen(path, "rb");

    if (file == NULL) {
        fprintf(stderr, "Could not open file \"%s\".\n", path);
        exit(74);
    }

    fseek(file, 0L, SEEK_END);
    size_t fileSize = ftell(file);
    rewind(file);

    char* buffer = (char*)malloc(fileSize + 1);

    if (buffer == NULL) {
        fprintf(stderr, "Not enough memory to read \"%s\".\n", path);
        exit(74);
    }
    size_t bytesRead = fread(buffer, sizeof(char), fileSize, file);

    if (bytesRead < fileSize) {
        fprintf(stderr, "Could not read file \"%s\".\n", path);
        exit(74);
    }

    buffer[bytesRead] = '\0';

    fclose(file);
    return buffer;
}

static void runFile(const char* path)
{
    char*           source = readFile(path);
    InterpretResult result = interpret(source);
    free(source);

    if (result == INTERPRET_COMPILE_ERROR)
        exit(65);
    if (result == INTERPRET_RUNTIME_ERROR)
        exit(70);
}

int main(int argc, char** argv)
{
    init_vm();
    // Chunk chunk;
    // init_chunk(&chunk);

    // int constant = add_constant(&chunk, 1.2);
    // write_chunk(&chunk, OP_CONSTANT, 123);
    // write_chunk(&chunk, constant, 123);
    // constant = add_constant(&chunk, 3.4);
    // write_chunk(&chunk, OP_CONSTANT, 123);
    // write_chunk(&chunk, constant, 123);
    // write_chunk(&chunk, OP_ADD, 123);

    // constant = add_constant(&chunk, 5.6);
    // write_chunk(&chunk, OP_CONSTANT, 123);
    // write_chunk(&chunk, constant, 123);

    // write_chunk(&chunk, OP_DIVIDE, 123);
    // write_chunk(&chunk, OP_NEGATIVE, 123);
    // write_chunk(&chunk, OP_RETURN, 123);

    // disassemble_chunk(&chunk, "test chunk");

    // interpret(&chunk);

    if (argc == 1) {
        // REPL
        repl();
    } else if (argc == 2) {
        runFile(argv[1]);
    } else {
        fprintf(stderr, "Usage: cmoss [path] \n");
    }
    free_vm();
    // free_chunk(&chunk);
    return 0;
}