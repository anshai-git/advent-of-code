#include <stdio.h>

const char *parse_file_path(int argc, char **args) {
  if (argc == 1) {
    fprintf(stderr,
            "File path missing\nUsage: %s <file path>\nExample: %s input.txt",
            args[0], args[0]);
    return NULL;
  }

  if (argc == 2) {
    return args[1];
  }

  if (argc > 2) {
    fprintf(stderr, "Invalid number of arguments.");
    return NULL;
  }

  return NULL;
}

FILE* read_file(const char* path) {
  FILE* file = fopen(path, "r");
  if (file == NULL) {
    fprintf(stderr, "Failed to open file: %s\n", path);
    return NULL;
  }
  return file;
}
