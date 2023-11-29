#include <stdio.h>

int value_of(char choice) {
  if (choice == 'A' || choice == 'X')
    return 1;
  if (choice == 'B' || choice == 'Y')
    return 2;
  return 3;
}

int play(int c1, int c2) {
  if (c1 == 1 && c2 == 3)
    return 0;
  if (c2 - c1 == -1)
    return 0;
  if (c2 - c1 == 0)
    return 3;
  return 6;
}

int generate(int todo, int opponent_choice) {
  if (todo == 3) {
    if (opponent_choice == 3)
      return 1;
    return opponent_choice + 1;
  }
  if (todo == 1) {
    if (opponent_choice == 1)
      return 3;
    return opponent_choice - 1;
  }
  return opponent_choice;
}

int main(int argc, char **argv) {
  if (argc == 1) {
    fprintf(stderr, "Usage: day_2 <input file path>\n");
    return 1;
  }

  char *path = argv[1];
  FILE *input = fopen(path, "r");
  if (input == NULL) {
    fprintf(stderr, "Failed tp open file: %s\n", path);
  }

  char buffer[8];
  int line = 0;
  int total_score = 0;
  while (fgets(buffer, sizeof buffer, input) != NULL) {
    char opponent_choice = buffer[0];
    char suggested_choice = buffer[2];

    // Part 1
    // int result = value_of(suggested_choice) +
    //              play(value_of(opponent_choice), value_of(suggested_choice));

    // Part 2
    int choice = generate(value_of(suggested_choice), value_of(opponent_choice));
    int result = choice + play(value_of(opponent_choice), choice);
    total_score += result;

    printf("%d %d %d, result: %d\n", value_of(opponent_choice),
           value_of(suggested_choice), choice, result);
  }

  printf("%d\n", total_score);

  return 0;
}
