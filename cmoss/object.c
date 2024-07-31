#include "object.h"
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include "hashtable.h"
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

static uint32_t hash_string(const char* key, int length)
{
    uint32_t hash = 2166136261u;
    for (int i = 0; i < length; i++) {
        hash ^= (uint8_t)key[i];
        hash *= 16777619;
    }
    return hash;
}

static ObjString* allocate_string(char* heap_chars, int length,
                                  uint32_t hash)
{
    ObjString* string = ALLOCATE_OBJ(ObjString, OBJ_STRING);
    string->chars = heap_chars;
    string->length = length;
    string->hash = hash;

    table_set(&vm.strings, string, NIL_VAL);
    return string;
}
ObjString* copy_string1(const char* chars, int length)
{
    char* heap_chars = ALLOCATE(char, length + 1);
    memcpy(heap_chars, chars, length);
    heap_chars[length] = '\0';
    uint32_t hash = hash_string(chars, length);

    ObjString* interned =
        table_find_string(&vm.strings, chars, length, hash);
    if (interned != NULL)
        return interned;

    return allocate_string(heap_chars, length, hash);
}

ObjString* take_string(char* chars, int length)
{
    uint32_t   hash = hash_string(chars, length);
    ObjString* interned =
        table_find_string(&vm.strings, chars, length, hash);
    if (interned != NULL) {
        FREE_ARRAY(char, chars, length + 1);
        return interned;
    }

    return allocate_string(chars, length, hash_string(chars, length));
}