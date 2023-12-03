#include <ctype.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

#include "args.h"
#include "files.h"

int max(int a, int b) {
  return a > b ? a : b;
}

int min(int a, int b) {
  return a < b ? a : b;
}

bool is_symbol(char c) {
  return !isdigit(c) && c != '.' && c != '\n';
}

bool find_symbol(int rs, int re, int cs, int ce, char buffer[][1024]) {
  for (int i = rs; i <= re; i++) {
    for (int j = cs; j <= ce; j++) {
      if (is_symbol(buffer[i][j])) {
        return true;
      }
    }
  }
  return false;
}

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

  while (++line_index < num_of_lines) {
    char *current_char = buffer[line_index];

    int row_start = max(line_index - 1, 0);
    int row_end = min(num_of_lines - 1, line_index + 1);
    int col_start = -1;
    int col_end = -1;

    int line_length = strlen(current_char);
    uint64_t current_number = 0;
    while (++col_index < line_length - 1 || current_number != 0) {
      if (isdigit(current_char[col_index])) {
        if (col_start == -1) col_start = max(0, col_index-1);
        col_end = min(line_length - 1, col_index + 1);
        current_number = current_number * 10 + (current_char[col_index] - '0');
      } else {
        if (current_number != 0) {
          if (find_symbol(row_start, row_end, col_start, col_end, buffer)) {
            sum += current_number;
          }
          current_number = 0;
          col_start = -1;
        }
      }
    }
    col_index = -1;
  }

  printf("%ld\n", sum);
  fclose(file);
  return 0;
}
