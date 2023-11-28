#ifndef elves_h_
#define elves_h_

#include <stdint.h>

typedef struct {
  uint32_t* totals;
  uint8_t size;
} Elves;

void init_elves(Elves* elves, uint8_t size);
void insert_total(Elves* elves, uint32_t total);
uint32_t sum_elves(Elves* elves);
void print_elves(Elves* elves);
void free_elves(Elves* elves);

#endif
