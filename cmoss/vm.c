#include "vm.h"
#include <stdint.h>
#include <stdio.h>
#include "chunk.h"
#include "compiler.h"
#include "debug.h"
#include "value.h"

VM vm;

static void reset_stack()
{
    vm.stackTop = vm.stack;
}
void init_vm()
{
    reset_stack();
}

void free_vm() {}

void push(Value value)
{
    *vm.stackTop = value;
    ++vm.stackTop;
}

Value pop()
{
    --vm.stackTop;
    return *vm.stackTop;
}

static InterpretResult run()
{
#define READ_BYTE() (*vm.ip++)
#define READ_CONSTANT() (vm.chunk->constants.values[READ_BYTE()])
#define BINARY_OP(op)                                                      \
    do {                                                                   \
        double b = pop();                                                  \
        double a = pop();                                                  \
        push(a op b);                                                      \
    } while (false)

    for (;;) {
#ifdef DEBUG_TRACE_EXECUTION
        printf("==========stack==========\n");
        for (Value* slot = vm.stack; slot < vm.stackTop; slot++) {
            printf("[ ");
            print_value(*slot);
            printf(" ]");
        }
        printf("\n");
        disassemble_instruction(vm.chunk, (int)(vm.ip - vm.chunk->code));
#endif
        uint8_t instruction;
        switch (instruction = READ_BYTE()) {
        case OP_RETURN: {
            print_value(pop());
            printf("\n");
            return INTERPRET_OK;
        }

        case OP_CONSTANT: {
            Value constant = READ_CONSTANT();
            push(constant);
            printf("\n");
            break;
        }
        case OP_ADD:
            BINARY_OP(+);
            break;
        case OP_SUBTRACT:
            BINARY_OP(-);
            break;
        case OP_MULTIPLY:
            BINARY_OP(*);
            break;
        case OP_DIVIDE:
            BINARY_OP(/);
            break;
        case OP_NEGATIVE: {
            push(-pop());
            break;
        }
        }
    }

#undef READ_BYTE
#undef READ_CONSTANT
#undef BINARY_OP
}

// InterpretResult interpret(Chunk* chunk)
// {
//     vm.chunk = chunk;
//     vm.ip = vm.chunk->code;
//     return run();
// }

InterpretResult interpret(char* source)
{
    Chunk chunk;
    init_chunk(&chunk);
    if (!compile(source, &chunk)) {
        free_chunk(&chunk);
        return INTERPRET_COMPILE_ERROR;
    }

    vm.chunk = &chunk;
    vm.ip = vm.chunk->code;

    InterpretResult result = run();
    free_chunk(&chunk);
    return result;
}
