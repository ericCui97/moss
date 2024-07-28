#include "object.h"
#include <stdlib.h>
#include <string.h>
#include "memory.h"
#include "value.h"
#include "vm.h"
// type为 分配的类型 object_type 为这个value结构体的type字段
#define ALLOCATE_OBJ(type, object_type)                                    \
    (type*)allocate_object(sizeof(type), object_type)

static Obj* allocate_object(size_t size, ObjType type)
{
    Obj* object = (Obj*)reallocate(NULL, 0, size);
    object->type = type;
    object->next = vm.objects;
    vm.objects = object;  // insert object linkedList into vm
    return object;
}

static ObjString* allocate_string(char* heap_chars, int length)
{
    ObjString* string = ALLOCATE_OBJ(ObjString, OBJ_STRING);
    string->chars = heap_chars;
    string->length = length;
    return string;
}
ObjString* copy_string1(const char* chars, int length)
{
    char* heap_chars = ALLOCATE(char, length + 1);
    memcpy(heap_chars, chars, length);
    heap_chars[length] = '\0';

    return allocate_string(heap_chars, length);
}

ObjString* take_string(char* chars, int length)
{
    return allocate_string(chars, length);
}