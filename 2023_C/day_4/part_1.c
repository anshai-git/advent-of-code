#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

#include "args.h"
#include "files.h"

bool is_digit(char c) { return c >= '0' && c <= '9'; }

void advance(char **c) { (*c)++; }

void consume(char **buffer, char c) {
  while (**buffer != c) (*buffer)++;
  (*buffer)++; // Consume the character itself
}

void parse_winning_numbers(bool *winning_numbers, char **current_char) {
  uint64_t current_number = 0;
  while (**current_char != '|') {
    if (is_digit(**current_char)) {
      current_number = current_number * 10 + (**current_char) - '0';
    } else {
      if (current_number != 0) winning_numbers[current_number] = true;
      current_number = 0;
    }
    advance(current_char);
  }
}

uint64_t parse_own_numbers(char **current_char, bool *winning_numbers) {
  uint64_t point_value = 0;
  uint64_t current_number = 0;
  while (**current_char != '\0') {
    if (is_digit(**current_char)) {
      current_number = current_number * 10 + (**current_char) - '0';
    } else {
      if (current_number != 0) {
        if (winning_numbers[current_number] == true)
          point_value = (point_value == 0) ? 1 : (2 * point_value);

        current_number = 0;
      }
    }
    advance(current_char);
  }
  return point_value;
}

int main(int argc, char **argv) {
  const char *path = parse_file_path(argc, argv);
  if (path == NULL) return 1;

  FILE *file = read_file(path);
  if (file == NULL) return 1;

  bool winning_numbers[1024] = {0};
  uint64_t sum = 0;

  char buffer[1024];
  while (fgets(buffer, sizeof(buffer), file) != NULL) {
    char *current_char = buffer;
    consume(&current_char, ':');

    parse_winning_numbers(winning_numbers, &current_char);
    sum += parse_own_numbers(&current_char, winning_numbers);
    memset(winning_numbers, 0, sizeof(winning_numbers));
  }

  printf("SUM: %ld\n", sum);

  fclose(file);
  return 0;
}
