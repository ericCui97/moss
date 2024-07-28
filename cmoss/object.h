#pragma once
#include "common.h"
#include "value.h"

typedef enum {
    OBJ_STRING,
} ObjType;

#define OBJ_TYPE(value) (AS_OBJ(value)->type)
#define IS_STRING(value) (is_obj_type(value, OBJ_STRING))
// 将一个string obj value中的 obj指针拿到
#define AS_STRING(value) ((ObjString*)AS_OBJ(value))
// 拿到 string obj  中的 chars
#define AS_CSTRING(value) (((ObjString*)AS_OBJ(value))->chars)

struct Obj {
    ObjType type;
    Obj*    next;
};

struct ObjString {
    Obj obj;  // 这个要写在第一位，这样任意的obj派生类型如objstring都可以安全的强转为obj指针进行操作
    char* chars;
    int   length;
};
// value 在此处被调用多次，不能直接写到宏里面
static inline bool is_obj_type(Value value, ObjType type)
{
    return IS_OBJ(value) && AS_OBJ(value)->type == type;
}

ObjString* copy_string1(const char* chars, int length);

// marcos
ObjString* take_string(char* chars, int length);
