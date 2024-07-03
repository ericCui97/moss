#include "memory.h"
#include <stdlib.h>
#include <sys/_types/_null.h>

void* reallocate(void *pointer, size_t oldSize, size_t newSize){
    if(newSize == 0){
        free(pointer);
        return NULL;
    }


    void* result = realloc(pointer,newSize);
    if(result == NULL)exit(1);
    return result;
}