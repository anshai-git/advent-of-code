#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

#include "args.h"
#include "files.h"
#include "map_array.h"
#include "range.h"
#include "sd_map.h"

typedef struct {
  uint64_t start;
  uint64_t end;
} SeedRange;

void init_seed_range(SeedRange* range, int64_t range_start, int64_t range_end) {
  range->start = range_start;
  range->end = range_end;
}

typedef struct {
  uint8_t size;
  uint8_t used;
  SeedRange* values;
}RangeArray;

void init_seed_range_array(RangeArray* array) {
  array->values = malloc(sizeof(SeedRange));
  array->used = 0;
  array->size = 1;
}

void add_seed_range(RangeArray* array, SeedRange range) {
  if (array->size == array->used) {
    array->size *= 2;
    array->values = realloc(array->values, array->size * sizeof(SeedRange));
  }
  array->values[array->used++] = range;
}

void free_seed_range_array(RangeArray* array) {
  free(array->values);
  array->size = array->used = 0;
  array->values = NULL;
}

bool is_digit(char c) { return c >= '0' && c <= '9'; }

void parse_seeds(RangeArray* range_array, char **buffer) {
  int64_t number = 0;
  int64_t range_start = 0;
  int64_t range_end = 0;
  while ((**buffer != '\n' && **buffer != '\0') || number != 0) {
    if (is_digit(**buffer)) {
      number = number * 10 + (**buffer) - '0';
    } else {
      if (0 != number) {
        if (range_start == 0) {
          range_start = number;
        } else {
          range_end = range_start + number - 1;
        }
        if(range_start != 0 && range_end != 0) {
          SeedRange seed_range;
          init_seed_range(&seed_range, range_start, range_end);
          add_seed_range(range_array, seed_range);
          range_start = range_end = 0;
        }
      }
      number = 0;
    }
    (*buffer)++;
  }
}

uint64_t seed_sorter(SeedRange* r1, SeedRange* r2) {
  return r1->end - r2->end;
}

void sort_seeds(RangeArray* array) {
  qsort(array->values, array->used, sizeof(SeedRange), seed_sorter);
}

void resolve_intersections(RangeArray* array) {
  for (int i = 0; i < array->used - 1; i++) {
    if (array->values[i].end >= array->values[i+1].start) {
      array->values[i].end = array->values[i+1].start - 1;
    }
  }
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
  if (NULL == filename) return 1;

  FILE *file = read_file(filename);
  if (NULL == file) return 1;

  MapArray map_array;
  init_map_array(&map_array);

  RangeArray range_array;
  init_seed_range_array(&range_array);

  char buffer[2048];
  int line_number = 0;
  while (fgets(buffer, sizeof(buffer), file) != NULL) {
    char *current_char = buffer;
    if (line_number++ == 0) {
      parse_seeds(&range_array, &current_char);
      sort_seeds(&range_array);
      resolve_intersections(&range_array);
      printf("parsed seeds");
      continue;
    }

    if (1 == strlen(buffer)) continue; // Skip empty lines

    if (is_digit(*current_char)) {
      int64_t range_values[8] = {0};
      char* c = buffer;
      parse_numbers(range_values, &c);

      Range range;
      init_range(&range, range_values[1], range_values[0], range_values[2]);
      add_range(&map_array.maps[map_array.used - 1], range);
    } else {
      SDMap sd_map;
      init_sd_map(&sd_map);
      add_map(&map_array, sd_map);
    }
  }

  int64_t min = INT64_MAX;
  for(int i = 0; i < range_array.used; i++) {
    printf("Mapping range %d of %d\n", i, range_array.used);

    int64_t range_start = range_array.values[i].start;
    int64_t range_end = range_array.values[i].end;
    while (range_start++ <= range_end) {
      int64_t current_res = range_start;
      for (int j = 0; j < map_array.used; j++) {
        current_res = resolve_map(&map_array.maps[j], current_res);
      }
      if(current_res < min) min = current_res;
    }
  }

  printf("\nMIN: %ld\n", min);
  // print_map_array(&map_array);
  free_seed_range_array(&range_array);
  free_map_array(&map_array);
  fclose(file);
  return 0;
}
