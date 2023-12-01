#include <stdio.h>

FILE *read_file(const char *path) {
  FILE *file = fopen(path, "r");
  if (file == NULL) {
    fprintf(stderr, "[ERROR] Failed to read file: %s\n", path);
  }
  return file;
}
