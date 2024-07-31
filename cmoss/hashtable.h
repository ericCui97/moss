#pragma once

#include "value.h"

typedef struct {
    ObjString* key;
    Value      value;
} Entry;

typedef struct {
    int    capacity;
    int    count;
    Entry* entries;

} Table;

void init_table(Table* table);

void free_table(Table* table);

void table_add_all(Table* from, Table* to);

bool table_get(Table* table, ObjString* key, Value* value);

bool table_set(Table* table, ObjString* key, Value value);

bool table_delete(Table* table, ObjString* key);

ObjString* table_find_string(Table* table, const char* str, int length,
                             uint32_t hash);