#include "map_array.h"

#include <stdio.h>

#include "sd_map.h"
#include <stdlib.h>

void init_map_array(MapArray *array) {
  array->maps = malloc(sizeof(SDMap));
  array->size = 1;
  array->used = 0;
}

void add_map(MapArray *array, SDMap map) {
  if (array->used == array->size) {
    array->size *= 2;
    array->maps = realloc(array->maps, array->size * sizeof(SDMap));
  }
  array->maps[array->used++] = map;
}

void free_map_array(MapArray *array) {
  for (int i = 0; i < array->used; i++) {
    free_sd_map(&array->maps[i]);
  }
  free(array->maps);
  array->used = array->size = 0;
  array->maps = NULL;
}

void print_map_array(MapArray* array) {
  for (int i = 0; i < array->used; i++) {
    // Each Map
    for (int j = 0; j < array->maps[i].used; j++) {
      // Each Range
      Range* range = &array->maps[i].ranges[j];
      printf("destination: %ld source: %ld length: %ld\n", range->destination, range->source, range->length);
    }
  }
  printf("\n");
}
