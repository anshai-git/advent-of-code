#include <ctype.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "args.h"
#include "files.h"

int max(int a, int b) { return a > b ? a : b; }
int min(int a, int b) { return a < b ? a : b; }
bool is_symbol(char c) { return !isdigit(c) && c != '.' && c != '\n'; }
bool is_star(char c) { return c == '*'; }

int main(int argc, char **argv) {
  const char *path = parse_file_path(argc, argv);
  if (path == NULL) return 1;

  FILE *file = read_file(path);
  if (file == NULL) return 1;

  char buffer[1024][1024];
  int line_index = -1;
  int col_index = -1;
  uint64_t sum = 0;
  int num_of_lines = 0;

  while (fgets(buffer[num_of_lines], sizeof(buffer[num_of_lines]), file) != NULL) {
    num_of_lines++;
  }

  uint64_t stars[20000][2] = {{0}};
  uint64_t star_indexes[1024] = {0};
  uint64_t num_of_stars = 0;

  while (++line_index < num_of_lines) {
    const char *current_char = buffer[line_index];
    const int row_start = max(line_index - 1, 0);
    const int row_end = min(num_of_lines - 1, line_index + 1);
    int col_start = -1;
    int col_end = -1;

    const int line_length = strlen(current_char) - 1;
    uint64_t current_number = 0;
    while (++col_index < line_length || current_number != 0) {
      if (isdigit(current_char[col_index])) {
        if (col_start == -1) col_start = max(0, col_index - 1);
        col_end = min(line_length, col_index + 1);
        current_number = current_number * 10 + (current_char[col_index] - '0');
      } else {
        if (current_number != 0) {
          for (int i = row_start; i <= row_end; i++) {
            for (int j = col_start; j <= col_end; j++) {
              if (is_star(buffer[i][j])) {
                const uint64_t index = i * line_length + j;
                if (stars[index][0] == 0) {
                  star_indexes[num_of_stars++] = index;
                }
                stars[index][0] += 1;
                if (stars[index][1] == 0) {
                  stars[index][1] = current_number;
                } else {
                  stars[index][1] *= current_number;
                }
              }
            }
          }
          current_number = 0;
          col_start = -1;
        }
      }
    }
    col_index = -1;
  }

  for (int i = 0; i < num_of_stars; i++) {
    const uint64_t index = star_indexes[i];
    if (stars[index][0] == 2) {
      sum += stars[index][1];
    }
  }

  printf("%ld\n", sum);
  fclose(file);
  return 0;
}
