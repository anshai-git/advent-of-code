#include <stdio.h>
#include <stdlib.h>

#include "elves.h"

int main(int argc, char **argv) {
  if (argc == 1) {
    fprintf(stderr, "Usage: day_1 <input file path>\n");
    return 1;
  }

  char *path = argv[1];
  FILE *input = fopen(path, "r");
  if (input == NULL) {
    fprintf(stderr, "Failed to read file: %s\n", path);
    return 1;
  }

  Elves elves;
  init_elves(&elves, 3);

  char buf[8];
  char *endptr;
  int total = 0;

  while (fgets(buf, sizeof(buf), input)) {
    total += strtoul(buf, &endptr, 10);
    if (buf[0] == '\n') { // We are between 2 elves
      insert_total(&elves, (uint32_t) total);
      total = 0;
    }
    // print_elves(&elves);
  }

  printf("Sum of first [%d] elves: %d\n", elves.size, sum_elves(&elves));

  free_elves(&elves);
  fclose(input);
  return 0;
}
