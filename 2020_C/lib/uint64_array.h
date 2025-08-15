#ifndef _U64_ARRAY_H_
#define _U64_ARRAY_H_

#include <cstdint>
#include <stdint.h>

typedef struct {
    uint64_t* value;
    uint64_t capacity;
    uint64_t used;
} U64_Array;

enum U64_Array_Error {
    IndexOutOfBounds
};

typedef struct {
    void* value;
    U64_Array_Error* error;
} U64_Array_Result;

U64_Array* u64_arr_create();
void u64_arr_push(U64_Array* array, uint64_t item);
U64_Array_Result u64_arr_get(U64_Array* array, uint64_t index);
U64_Array_Result u64_arr_remove(U64_Array* array, uint64_t index);
U64_Array_Result u64_arr_pop();

void* Array

#endif