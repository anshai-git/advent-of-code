#include <stdint.h>
#include <stdio.h>
#include <stdbool.h>

#include "args.h"
#include "files.h"

bool is_digit(char c);
uint64_t parse_number(char* buffer);
uint64_t count_possibilities(const uint64_t time, const uint64_t record);

int main(int argc, char** argv) {
  const char* filename = parse_file_path(argc, argv);
  if (NULL == filename) return 1;

  FILE* file = read_file(filename);
  if (NULL == file) return 1;

  char buffer[1024];

  fgets(buffer, sizeof(buffer), file);
  uint64_t time = parse_number(buffer);

  fgets(buffer, sizeof(buffer), file);
  uint64_t record = parse_number(buffer);

  uint64_t result = count_possibilities(time, record);

  printf("RESULT: %ld\n", result);
  return 0;
}

uint64_t count_possibilities(const uint64_t time, const uint64_t record) {
  uint64_t count = 0;
  for (uint64_t i = 0; i < time; i++) {
    if (record < (time - i) * i) count++;
  }
  return count == 0 ? 1 : count;
}

uint64_t parse_number(char* buffer) {
  uint64_t number = 0;
  while(*buffer != '\n') {
    if (is_digit(*buffer)) {
      number = number * 10 + (*buffer) - '0';
    }
    buffer++;
  }
  return number;
}

bool is_digit(char c) {
  return c >= '0' && c <= '9';
}
