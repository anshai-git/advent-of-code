#ifndef _MAP_ARRAY_H_
#define _MAP_ARRAY_H_

#include <stdint.h>

#include "sd_map.h"

typedef struct {
  uint8_t size;
  uint8_t used;
  SDMap* maps;
} MapArray;

void init_map_array(MapArray* array);
void add_map(MapArray* array, SDMap map);
void free_map_array(MapArray* array);
void print_map_array(MapArray* array);

#endif
