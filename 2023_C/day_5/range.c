#include <stdint.h>

#include "range.h"

void init_range(Range *range, int64_t source, int64_t destination, int64_t length) {
  range->source = source;
  range->destination = destination;
  range->length = length;
}
