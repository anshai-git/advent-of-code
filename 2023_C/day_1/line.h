#ifndef _line_h_
#define _line_h_

#include <stdint.h>

typedef struct {
  uint64_t raw_size;
  uint64_t raw_used;
  char* raw;

  uint64_t digits_size;
  uint64_t digits_used;
  char* digits;
} Line;

void init_line(Line* line);
void insert_digit(Line* line, char digit);
void insert_raw(Line* line, char c);
void free_line(Line* line);
void print_line(Line* line);

#endif
