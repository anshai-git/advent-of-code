#include <stdlib.h>
#include "sd_map.h"
#include "range.h"

void init_sd_map(SDMap *map) {
  map->ranges = malloc(sizeof(Range));
  map->size = 1;
  map->used = 0;
}

void add_range(SDMap* map, Range range) {
  if(map->used == map->size) {
    map->size *= 2;
    map->ranges = realloc(map->ranges, map->size *sizeof(Range));
  }
  map->ranges[map->used++] = range;
}

void free_sd_map(SDMap* map) {
  free(map->ranges);
  map->size = map->used = 0;
  map->ranges = NULL;
}

int64_t resolve_map(SDMap* sd_map, int64_t num) {
  for (int i = 0; i < sd_map->used; i++) {
    const Range* range = &sd_map->ranges[i];
    if (num >= range->source && num <= (range->source + range->length - 1)) {
      return (range->destination - range->source) + num;
    }
  }
  return num;
}
