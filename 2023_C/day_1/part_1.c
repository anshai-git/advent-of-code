#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

#include "files.h"
#include "line.h"

typedef struct {
  uint64_t size;
  uint64_t used;
  Line *lines;
} CalibrationDocument;

void insert_line(CalibrationDocument *calibration_document, Line line) {
  if (calibration_document->used == calibration_document->size) {
    calibration_document->size *= 2;
    calibration_document->lines = realloc(
        calibration_document->lines, calibration_document->size * sizeof(Line));
  }
  calibration_document->lines[calibration_document->used++] = line;
}

void add_new_line(CalibrationDocument *calibration_document) {
  Line line;
  init_line(&line);
  insert_line(calibration_document, line);
}

void init_calibration_document(CalibrationDocument *calibration_document) {
  calibration_document->lines = malloc(sizeof(Line));
  calibration_document->size = 1;
  calibration_document->used = 0;
  add_new_line(calibration_document);
}

void free_calibration_document(CalibrationDocument *calibration_document) {
  for (int i = 0; i < calibration_document->used; i++) {
    free_line(&calibration_document->lines[i]);
  }
  free(calibration_document->lines);
  calibration_document->lines = NULL;
  calibration_document->used = calibration_document->size = 0;
}

void print_calibration_document(CalibrationDocument *cd) {
  for (int i = 0; i < cd->used; i++) {
    print_line(&cd->lines[i]);
  }
}

bool is_digit(char c) { return c > '0' && c <= '9'; }
bool is_newline(char c) { return c == '\n'; }

uint64_t parse_line(CalibrationDocument *cd, char *raw_line) {
  char current_char = '0';
  Line *line = &cd->lines[cd->used - 1];

  while (current_char != '\n') {
    current_char = *raw_line++;
    insert_raw(line, current_char);

    if (is_digit(current_char)) {
      insert_digit(line, current_char);
    }
  }

  uint64_t first_digit = line->digits[0] - '0';
  uint64_t last_digit = line->digits[line->digits_used - 1] - '0';
  return (first_digit * 10) + last_digit;
}

int main(int argc, const char **argv) {
  if (argc == 1) {
    fprintf(stderr, "Usage: %s <input file path>\n", argv[0]);
  }

  FILE *input_file = read_file(argv[1]);
  if (input_file == NULL)
    return 1;

  CalibrationDocument calibration_document;
  init_calibration_document(&calibration_document);

  char buffer[1024];
  uint64_t result = 0;
  while (fgets(buffer, sizeof(buffer), input_file) != NULL) {
    result += parse_line(&calibration_document, buffer);
    add_new_line(&calibration_document);
  }

  printf("%ld\n", result);
  free_calibration_document(&calibration_document);
  return 0;
}
