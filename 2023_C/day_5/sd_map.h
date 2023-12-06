#ifndef _SD_MAP_H_
#define _SD_MAP_H_

#include <stdlib.h>
#include <stdint.h>
#include <stdlib.h>

#include "range.h"

typedef struct {
  uint8_t size;
  uint8_t used;
  Range* ranges; 
} SDMap;

void init_sd_map(SDMap* map);
void add_range(SDMap* map, Range range);
int64_t resolve_map(SDMap* map, int64_t value);
void free_sd_map(SDMap* map);

#endif
