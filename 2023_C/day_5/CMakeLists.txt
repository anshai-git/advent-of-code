cmake_minimum_required(VERSION 3.27)
project(part C)

set(CMAKE_C_STANDARD 99)

add_executable(part2
        part_2.c
        map_array.c
        map_array.h
        sd_map.c
        sd_map.h
        range.c
        range.h
        args.h
        args.c
        files.h
        files.c)