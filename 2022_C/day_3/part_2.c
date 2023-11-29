#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

#include "files.h"
#include "utils.h"

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
  int set_count = 1;

  char buffer[64];
  while (fgets(buffer, sizeof buffer, input_file)) {
    char *c = &buffer;
    uint8_t length = (strlen(buffer) - 1);

    while (length-- > 0) {
      int current_char_value = int_value(*c++);

      if (set_count == 1)
        map[current_char_value] = set_count;

      if (set_count == 2 && map[current_char_value] > 0)
        map[current_char_value] = set_count;

      if (set_count == 3 && map[current_char_value] > 1) {
        final_sum += current_char_value;
        break;
      }

      if (set_count++ == 3) {
        memset(map, 0, 1024);
        set_count = 1;
      }
    }
  }

  printf("SOLUTION: %ld\n", final_sum);

  fclose(input_file);
  return 0;
}
