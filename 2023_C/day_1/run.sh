#!/bin/sh

if [ -z "$1" ]; then
  echo "Usage: $0 <part index>"
  echo "Ex: $0 1"
  exit 1
fi

output="part$1"

gcc part_$1.c files.c line.c -o "$output"

if [ $? -eq 0 ]; then
  echo "Compilation succesfull."
  "./$output" input_$1.txt
else
  echo "Compilation failed."
fi

