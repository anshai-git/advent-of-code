#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

#include "args.h"
#include "files.h"
#include "map_array.h"
#include "range.h"
#include "sd_map.h"

bool is_digit(char c) { return c >= '0' && c <= '9'; }

void parse_seeds(int64_t *seeds, int *count, char **buffer) {
  int64_t seed_count = 0;
  int64_t number = 0;
  while ((**buffer != '\n' && **buffer != '\0') || number != 0) {
    if (is_digit(**buffer)) {
      number = number * 10 + (**buffer) - '0';
    } else {
      if (0 != number) {
        seeds[seed_count++] = number;
      }
      number = 0;
    }
    (*buffer)++;
  }
  *count = seed_count;
}

void parse_numbers(int64_t* numbers, char** current_char) {
  int64_t number = 0;
  uint8_t numbers_count = 0;

  while(**current_char != '\n' || number != 0) {
    if (is_digit(**current_char)) {
      if (number == 0 && **current_char == '0') {
        numbers_count++;
        (*current_char)++;
        continue;
      }
      number = number * 10 + (**current_char) - '0';
    } else {
      if (number != 0) numbers[numbers_count++] = number;
      number = 0;
    }
    (*current_char)++;
  }
}

int main(int argc, char **argv) {
  const char *filename = parse_file_path(argc, argv);
  if (NULL == filename)
    return 1;

  FILE *file = read_file(filename);
  if (NULL == file)
    return 1;

  int64_t* seeds = malloc(32 * sizeof(int64_t));
  int seeds_count = 0;

  MapArray map_array;
  init_map_array(&map_array);

  char buffer[2048];
  int line_number = 0;
  while (fgets(buffer, sizeof(buffer), file) != NULL) {
    char *current_char = buffer;
    if (line_number++ == 0) {
      parse_seeds(seeds, &seeds_count, &current_char);
      continue;
    }

    if (1 == strlen(buffer)) continue;

    if (is_digit(*current_char)) {
      int64_t numbers[8] = {0};
      char* c = buffer;
      parse_numbers(numbers, &c);

      Range range;
      init_range(&range, numbers[1], numbers[0], numbers[2]);
      add_range(&map_array.maps[map_array.used - 1], range);
    } else {
      SDMap sd_map;
      init_sd_map(&sd_map);
      add_map(&map_array, sd_map);
    }
  }

  int64_t min = INT64_MAX;
  for (int64_t i = 0; i < seeds_count; i++) {
    int64_t result = seeds[i];
    for (int64_t j = 0; j < map_array.used; j++) {
      result = resolve_map(&map_array.maps[j], result);
    }
    if (min > result) min = result;
  }

  printf("MIN: %ld\n", min);
  // print_map_array(&map_array);
  free_map_array(&map_array);
  free(seeds);
  fclose(file);
  return 0;
}
