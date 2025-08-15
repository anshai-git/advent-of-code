#include <stdio.h>
#include "common.h"

int main(int argc, char** argv) {
  const char* filename = parse_file_path(argc, argv);
  if (NULL == filename) {
    fprintf(stderr, "Invalid filename.\nUsage: <program> <filename>\n");
    return 1;
  }

  FILE* file = read_file(filename);
  if (NULL == file) {
    fprintf(stderr, "Failed to read file.");
    return 1;
  }

  

  return 0;
}
