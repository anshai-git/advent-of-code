#ifndef _RANGE_H_
#define _RANGE_H_

#include <stdint.h>

typedef struct {
  int64_t source;
  int64_t destination;
  int64_t length;
} Range;

void init_range(Range* range, int64_t source, int64_t destination, int64_t length);

#endif // _RANGE_H_
