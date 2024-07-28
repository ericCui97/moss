#pragma once
#include "common.h"

// typedef double Value;

typedef enum {
    VAL_BOOL,
    VAL_NIL,
    VAL_NUMBER,
    VAL_OBJ,
} ValueType;

typedef struct Obj Obj;
// 前向声明，声明不需要知道具体类型（也就是不需要在这里开辟内存）
// 只是告诉编译器这个obj结构体在别处声明了，这里是为了避免object和value循环引用
typedef struct ObjString ObjString;
typedef struct {
    ValueType type;
    union {
        bool   boolean;
        double number;
        Obj*   obj;

    } as;  // 占8个字节 union类型
} Value;

#define AS_BOOL(value) ((value).as.boolean)
#define AS_NUMBER(value) ((value).as.number)
#define AS_OBJ(value) ((value).as.obj)

#define BOOL_VAL(value) ((Value){VAL_BOOL, {.boolean = value}})
#define NIL_VAL ((Value){VAL_NIL, {.number = 0}})
#define NUMBER_VAL(value) ((Value){VAL_NUMBER, {.number = value}})
#define OBJ_VAL(object) ((Value){VAL_OBJ, {.obj = (Obj*)object}})

#define IS_BOOL(value) ((value).type == VAL_BOOL)
#define IS_NIL(value) ((value).type == VAL_NIL)
#define IS_NUMBER(value) ((value).type == VAL_NUMBER)
#define IS_OBJ(value) ((value).type == VAL_OBJ)
typedef struct{
    int capacity;
    int count;
    Value* values;
} ValueArray;

void init_value_array(ValueArray* array);

void write_value_array(ValueArray* array, Value value);

void free_value_array(ValueArray* array);

void print_value(Value value);

bool values_equal(Value a, Value b);

void print_object(Value value);