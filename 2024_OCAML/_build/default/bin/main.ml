(* let read_file_lines filename =
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

let split_and_extract line =
  let parts = String.split_on_char ' ' (String.trim line) in
  let numbers = List.filter (fun s -> s <> "") parts in
  match numbers with
  | [] -> None
  | [left; right] -> Some (int_of_string left, int_of_string right)
  | _ -> None

let sum_of_differences lines =
  let differences = List.fold_left (fun acc line ->
    match split_and_extract line with
    | Some (first, last) -> acc + (last - first)
    | None -> acc
  ) 0 lines in
  differences

let part_2 lines =
  let result = List.fold_left (fun (left, right) line ->
    match split_and_extract line with
    | Some (l, r) -> (l :: left, r :: right)
    | None -> (left, right)
  ) ([], []) lines in
  result

let () =
  let lines = read_file_lines "example.txt" in
  let total_difference = sum_of_differences lines in
  Printf.printf "Total sum of differences: %d\n" total_difference

let () =
  let lines = read_file_lines "example.txt" in
  let (l, r) = part_2 lines in *)




let nums = [1; 2; 3; 4; 5; 6; 7]

(* List Length *)
let rec list_count list =
  match list with
  | [] -> 0
  | _ :: rest -> 1 + list_count rest

(* Sum List *)
let rec list_sum list =
  match list with
  | [] -> 0
  | h :: rest -> h + list_sum rest

(* Count even numbers in list *)
let rec count_even list =
  match list with
  | [] -> 0
  | h :: rest -> (if (h mod 2 == 0) then 1 else 0) + count_even rest

(* Compute the nth fibonacci number *)
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

let count = list_count nums
let () = Printf.printf "List Length: %d\n" count

let sum = list_sum nums
let () = Printf.printf "List Sum: %d\n" sum

let even_nums_count = count_even nums
let () = Printf.printf "Even numbers count: %d\n" even_nums_count
