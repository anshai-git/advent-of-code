cmake_minimum_required(VERSION 3.27)
project(part1 C)

set(CMAKE_C_STANDARD 99)

add_executable(part1 part_1.c
        args.h
        args.c
        files.h
        files.c)