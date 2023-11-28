#include "elves.h"
#include <stdio.h>
#include <stdlib.h>

void init_elves(Elves *elves, uint8_t size) {
  elves->totals = malloc(size * sizeof(uint32_t));
  elves->size = size;
}

void insert_total(Elves *elves, uint32_t total) {
  uint32_t current = total;
  for (int i = 0; i < elves->size; i++) {
    if (elves->totals[i] < current) {
      uint32_t temp = current;
      current = elves->totals[i];
      elves->totals[i] = temp;
    }
  }
}

uint32_t sum_elves(Elves* elves) {
  uint32_t sum = 0;
  for(int i = 0; i < elves->size; i++) {
    sum += elves->totals[i];
  }
  return sum;
}

void print_elves(Elves* elves) {
  for (int i = 0; i < elves->size; i++) {
    printf("[%d]: %d\n", i, elves->totals[i]);
  }
}

void free_elves(Elves *elves) {
  free(elves->totals);
  elves->totals = NULL;
  elves->size = 0;
}
