let nums = [1; 2; 3; 4; 5; 6; 7]

(* >> List Length *)
let rec list_length list =
  match list with
  | [] -> 0
  | _ :: rest -> 1 + list_length rest

let length = list_length nums
let () = Printf.printf "List Length: %d\n" length
(* << List Length *)

(* >> Sum List *)
let rec list_sum list =
  match list with
  | [] -> 0
  | h :: rest -> h + list_sum rest

let sum = list_sum nums
let () = Printf.printf "List Sum: %d\n" sum
(* << Sum List *)

(* >> Count even numbers in list *)
let rec count_even list =
  match list with
  | [] -> 0
  | h :: rest -> (if (h mod 2 == 0) then 1 else 0) + count_even rest

let even_nums_count = count_even nums
let () = Printf.printf "Even numbers count: %d\n" even_nums_count
(* << Count even numbers in list *)

(* >> Compute the nth fibonacci number *)
let rec fib n =
  match n with
  | 1 -> 1
  | 2 -> 1
  | n -> fib (n - 1) + fib (n - 2)

let first_fib = fib 1
let () = Printf.printf "First Fib: %d\n" first_fib

let second_fib = fib 2
let () = Printf.printf "Second Fib: %d\n" second_fib

let nth_fib = fib 3
let () = Printf.printf "Nth(3) Fib: %d\n" nth_fib

let nth_fib = fib 4
let () = Printf.printf "Nth(4) Fib: %d\n" nth_fib
(* << Compute the nth fibonacci number *)

(* >> Reverse a string *)
let rec string_reverse str =
  match str with
  | "" -> ""
  | _  -> string_reverse (String.sub str 1 (String.length str - 1)) ^ String.sub str 0 1

let () =
  let some_text = "stressed" in
  let reversed = string_reverse some_text in
  Printf.printf "Reversed: %s\n" reversed
(* << Reverse a string *)

(* >> Day 1 Part 1*)
let read_file_lines filename =
  let channel = open_in filename in
  let rec read_lines acc =
    try
      let line = input_line channel in
      read_lines (line :: acc)
    with End_of_file ->
      close_in channel;
      List.rev acc
  in
  read_lines []

let rec print_lines lines =
  match lines with
  | head :: rest -> Printf.printf "%s" head;
    print_lines rest
  | [] -> ()

let split_and_extract_numbers s =
  let regex = Str.regexp "[ \t]+" in
  let parts = Str.split regex s in
  List.map int_of_string parts

let rec get_pairs s =


let left = [];
let right = [];

let rec process_pairs pairs =
 match pairs with
 |

let () =
  let lines = read_file_lines "example.txt" in
  print_lines lines
(* << Day 1 Part 1*)
