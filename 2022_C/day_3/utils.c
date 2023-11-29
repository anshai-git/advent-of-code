#include <stdint.h>

uint16_t int_value(char c) {
  return (uint16_t)c > 96 ? (uint16_t)c - 96 : (uint16_t)c - 38;
}
