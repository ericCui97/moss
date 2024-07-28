#include "memory.h"
#include <stdlib.h>
#include "object.h"
#include "value.h"
#include "vm.h"
void* reallocate(void *pointer, size_t oldSize, size_t newSize){
    if(newSize == 0){
        free(pointer);
        return NULL;
    }


    void* result = realloc(pointer,newSize);
    if(result == NULL)exit(1);
    return result;
}

void free_object(Obj* obj)
{
    switch (obj->type) {
    case OBJ_STRING: {
        ObjString* string = (ObjString*)obj;
        FREE_ARRAY(char, string->chars, string->length + 1);
        FREE(ObjString, 0);
        break;
    }
    }
}
void free_objects()
{
    Obj* object = vm.objects;
    while (object != NULL) {
        Obj* next = object->next;
        object = next;
        free_object(next);
    }
}