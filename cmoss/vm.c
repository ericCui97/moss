#include "vm.h"
#include "common.h"
#include "debug.h"

VM          vm;
static void resetStack()
{
    vm.stackTop = vm.stack;
}

static void freeStack() {}
void        initVM()
{
    resetStack();
}

void freeVM()
{
    freeStack();
}

static InterpretResult run()
{
#define READ_BYTE() (*vm.ip++)

    for (;;) {
#ifdef DEBUG_TRACE_EXECUTION
        printf("          ");
        for (Value* slot = vm.stack; slot < vm.stackTop; slot++) {
            printf("[ ");
            printValue(*slot);
            printf(" ]");
        }
        printf("\n");
        disassembleInstruction(vm.chunk, (int)(vm.ip - vm.chunk->code));
#endif
        uint8_t instruction;
        switch (instruction = READ_BYTE()) {
        case OP_RETURN: {
            printValue(pop());
            printf("\n");
            return INTERPRET_OK;
        }
        case OP_CONSTANT: {
            Value constant = READ_CONSTANT();
            push(constant);
            printValue(constant);
            printf("\n");
            break;
        }
        case OP_NEGATE:
            push(-pop());
            break;
        }
    }

#undef READ_BYTE
#undef READ_CONSTANT
}

InterpretResult interpret(Chunk* chunk)
{
    vm.chunk = chunk;
    vm.ip = vm.chunk->code;
    //   return run();
    return run();
}

void push(Value value)
{
    *vm.stackTop = value;
    vm.stackTop++;
}

Value pop()
{
    vm.stackTop--;
    return *vm.stackTop;
}
