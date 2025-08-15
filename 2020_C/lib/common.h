#ifndef _COMMON_H_
#define _COMMON_H_

#include <cstdint>
#include <stdio.h>

const char *parse_file_path(int argc, char **args);
FILE* read_file(const char* path);

#endif
