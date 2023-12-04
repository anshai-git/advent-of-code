#include <stdio.h>

FILE* read_file(const char* path) {
  FILE* file = fopen(path, "r");
  if (file == NULL) {
    fprintf(stderr, "Failed to open file: %s\n", path);
    return NULL;
  }
  return file;
}
