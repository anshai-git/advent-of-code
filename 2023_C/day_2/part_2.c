#include <ctype.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

#include "args.h"
#include "files.h"

typedef struct {
  uint8_t red;
  uint8_t green;
  uint8_t blue;
} Set;

void advance(char **c) { (*c)++; }

bool is_ending_char(char c) { return c == '\n' || c == ';' || c == '\0'; }

void clear_set(Set *set) { set->red = set->green = set->blue = 0; }

bool is_set_valid(const Set *set) {
  return set->red <= 12 && set->green <= 13 && set->blue <= 14;
}

void consume(char **buffer, char target) {
  while (**buffer != target && !is_ending_char(**buffer)) advance(buffer);
  if (!is_ending_char(**buffer)) advance(buffer);
}

uint8_t parse_digit(char **buffer) {
  if (**buffer == ' ') advance(buffer);

  uint8_t number = 0;
  while (isdigit(**buffer) && !is_ending_char(**buffer)) {
    number *= 10;
    number += (**buffer) - '0';
    advance(buffer);
  }
  return number;
}

char parse_color(char **buffer) {
  if (**buffer == ' ') advance(buffer);

  if (isalpha(**buffer) && !is_ending_char(**buffer)) {
    return **buffer;
  }

  return NULL;
}

void parse_set(Set *set, char **buffer) {
  while (!is_ending_char(**buffer)) {
    const uint8_t color_count = parse_digit(buffer);
    switch (parse_color(buffer)) {
    case 'r': {
      if (color_count > set->red) { set->red = color_count; }
    } break;
    case 'g': {
      if (color_count > set->green) { set->green = color_count; }
    } break;
    case 'b': {
      if (color_count > set->blue) { set->blue = color_count; }
    } break;
    default:
      break;
    }
    consume(buffer, ',');
  }
}

bool parse_game(Set *set, char **buffer) {
  while (!is_ending_char(**buffer)) {
    parse_set(set, buffer);
    if (**buffer == ';')
      advance(buffer);
  }
  return true;
}

int main(int argc, char **argv) {
  const char *path = parse_file_path(argc, argv);
  if (path == NULL)
    return 1;

  FILE *file = read_file(path);
  if (file == NULL)
    return 1;

  uint64_t result = 0;
  char buffer[2048] = {0};

  uint8_t game_index = 1;
  Set *set = malloc(sizeof(Set));

  while (fgets(buffer, sizeof(buffer), file) != NULL) {
    char *current_char = buffer;
    consume(&current_char, ':');
    parse_game(set, &current_char);
    result += set->red * set->green * set->blue;
    clear_set(set);
    game_index++;
  }

  printf("RESULT: %ld\n", result);

  free(set);
  set = NULL;
  return 0;
}
