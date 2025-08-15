#include "uint64_array.h"
#include "common.h"
#include <cstdlib>
#include <stdint.h>
#include <stdlib.h>

U64_Array* u64_arr_create() {
    U64_Array* array = malloc(sizeof(U64_Array));
    if (NULL == array) {
        free(array);
        return NULL;
    }

    array->value = malloc(sizeof(uint64_t));
    if (NULL == array->value) {
        free(array);
        return NULL;
    }

    array->capacity = 1;
    array->used = 0;
    return array;
}

void u64_arr_push(U64_Array* array, uint64_t item) {
    if (array->capacity == array->used) {
        array->capacity *= 2;
        array->value = realloc(array->value, array->capacity * sizeof(uint64_t));
    }
    array->value[array->used] = item;
    array->used += 1;
}

U64_Array_Result u64_arr_get(U64_Array* array, uint64_t index) {
    U64_Array_Result result;
    if (index < 0 || index > array->used) {
        return
    }
}

