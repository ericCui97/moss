#pragma once
#include "common.h"

// typedef double Value;

typedef enum {
    VAL_BOOL,
    VAL_NIL,
    VAL_NUMBER,
} ValueType;
typedef struct {
    ValueType type;
    union {
        bool   boolean;
        double number;
    } as;
} Value;

#define AS_BOOL(value) ((value).as.boolean)
#define AS_NUMBER(value) ((value).as.number)

#define BOOL_VAL(value) ((Value){VAL_BOOL, {.boolean = value}})
#define NIL_VAL ((Value){VAL_NIL, {.number = 0}})
#define NUMBER_VAL(value) ((Value){VAL_NUMBER, {.number = value}})

#define IS_BOOL(value) ((value).type == VAL_BOOL)
#define IS_NIL(value) ((value).type == VAL_NIL)
#define IS_NUMBER(value) ((value).type == VAL_NUMBER)
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