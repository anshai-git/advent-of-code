#include <stdint.h>
#include <stdio.h>
#include <stdbool.h>

#include "args.h"
#include "files.h"

bool is_digit(char c);
void parse_numbers(uint16_t* into, char* buffer);
uint64_t count_possibilities(const uint16_t time, const uint16_t record);

int main(int argc, char** argv) {
  const char* filename = parse_file_path(argc, argv);
  if (NULL == filename) return 1;

  FILE* file = read_file(filename);
  if (NULL == file) return 1;

  char buffer[1024];
  uint16_t time_array[1024] = {0};
  uint16_t distance_array[1024] = {0};

  fgets(buffer, sizeof(buffer), file);
  parse_numbers(time_array, buffer);

  fgets(buffer, sizeof(buffer), file);
  parse_numbers(distance_array, buffer);

  uint64_t result = 1;
  for (int i = 1; i < time_array[0]; i++) {
    const uint16_t time = time_array[i];
    const uint16_t record = distance_array[i];
    result *= count_possibilities(time, record);
  }

  printf("RESULT: %ld\n", result);
  return 0;
}

bool is_digit(char c) {
  return c >= '0' && c <= '9';
}

void parse_numbers(uint16_t* into, char* buffer) {
  size_t count = 1;
  uint16_t number = 0;
  while((*buffer != '\n' && *buffer != '\0') || number != 0) {
    if (is_digit(*buffer)) {
      number = number * 10 + (*buffer) - '0';
    } else {
      if (number != 0) {
        into[count++] = number;
      }
      number = 0;
    }
    buffer++;
  }
  into[0] = count;
}

uint64_t count_possibilities(const uint16_t time, const uint16_t record) {
  uint16_t count = 0;
  for (uint16_t i = 0; i < time; i++) {
    if (record < (time - i) * i) count++;
  }
  return count == 0 ? 1 : count;
}
