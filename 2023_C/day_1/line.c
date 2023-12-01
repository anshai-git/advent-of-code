#include "line.h"
#include <stdio.h>
#include <stdlib.h>

void init_line(Line *line) {
  line->raw = malloc(sizeof(char));
  line->raw_size = 1;
  line->raw_used = 0;

  line->digits = malloc(sizeof(char));
  line->digits_size = 1;
  line->digits_used = 0;
}

void free_line(Line *line) {
  free(line->digits);
  free(line->raw);
  line->raw = line->digits = NULL;
  line->digits_used = line->digits_size = 0;
  line->raw_used = line->raw_size = 0;
}

void insert_raw(Line *line, char c) {
  if (line->raw_size == line->raw_used) {
    line->raw_size *= 2;
    line->raw = realloc(line->raw, line->raw_size * sizeof(char));
  }
  line->raw[line->raw_used++] = c;
}

void insert_digit(Line *line, char digit) {
  if (line->digits_size == line->digits_used) {
    line->digits_size *= 2;
    line->digits = realloc(line->digits, line->digits_size * sizeof(char));
  }
  line->digits[line->digits_used++] = digit;
}

void print_line(Line *line) {
  for (int i = 0; i < line->raw_used; i++) {
    printf("%c", line->raw[i]);
  }
  for (int i = 0; i < line->digits_used; i++) {
    printf("%c", line->digits[i]);
  }
  printf("\n\n");
}
