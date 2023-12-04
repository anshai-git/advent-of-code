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

void parse_winninw_numbers(bool* winning_numbers, char** current_char) {
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

// Returns the number of matches found
uint64_t parse_own_numbers(char** current_char, bool* winning_numbers) {
    uint64_t current_matches = 0;
    uint64_t current_number = 0;
    while (**current_char != '\0') {
      if (is_digit(**current_char)) {
        current_number = current_number * 10 + (**current_char) - '0';
      } else {
        if (current_number != 0) {
          if (winning_numbers[current_number] == true) current_matches++;
          current_number = 0;
        }
      }
      advance(current_char);
    }
    return current_matches;
}

int main(int argc, char **argv) {
  const char *path = parse_file_path(argc, argv);
  if (path == NULL) return 1;

  FILE *file = read_file(path);
  if (file == NULL) return 1;

  bool winning_numbers[1024] = {0};
  uint64_t sum = 0;
  uint8_t card_index = 1;
  uint64_t instances_by_index[1024] = {[0 ... (1023)] = 1};

  char buffer[1024];
  while (fgets(buffer, sizeof(buffer), file) != NULL) {
    char *current_char = buffer;
    consume(&current_char, ':');

    parse_winninw_numbers(winning_numbers, &current_char);
    uint64_t current_matches = parse_own_numbers(&current_char, winning_numbers);

    for (int i = card_index+1; i <= card_index + current_matches; i++) {
      instances_by_index[i] = instances_by_index[i] + instances_by_index[card_index];
    }

    sum += instances_by_index[card_index];
    memset(winning_numbers, 0, sizeof(winning_numbers));
    card_index++;
  }

  printf("SUM: %ld\n", sum);

  fclose(file);
  return 0;
}
