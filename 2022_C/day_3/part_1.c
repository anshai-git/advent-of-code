#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

#include "files.h"
#include "utils.h"

void fill_map(int length, char *buffer, int *map) {
  while (length-- > 0) map[int_value(*buffer++)] = 1;
}

uint64_t calculate_sum(int length, char *buffer, int *map) {
  uint64_t sum = 0;
  while (length-- > 0) {
    uint16_t current_char_value = int_value(*buffer++);
    if (map[current_char_value] == 1) {
      sum += current_char_value;
      break;
    }
  }
  return sum;
}

int main(int argc, char **argv) {
  if (argc == 1) {
    fprintf(stderr, "Usage: %s <input file path>\n", argv[0]);
    return 1;
  }

  FILE *input_file = read_file(argv[1]);
  if (input_file == NULL)
    return 1;

  int map[1024] = {0};
  uint64_t final_sum = 0;
  char buffer[64];

  while (fgets(buffer, sizeof buffer, input_file)) {
    uint8_t length = (strlen(buffer) - 1);

    fill_map(length / 2, buffer, map);
    final_sum += calculate_sum(length / 2, &buffer[length / 2], map);
    memset(map, 0, 1024);
  }

  printf("SOLUTION: %ld\n", final_sum);

  fclose(input_file);
  return 0;
}
