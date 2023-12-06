cmake_minimum_required(VERSION 3.27)
project(part2 C)

set(CMAKE_C_STANDARD 99)

add_executable(part2 part_2.c
        args.h
        args.c
        files.h
        files.c)
