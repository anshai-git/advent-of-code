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
  let (l, r) = part_2 lines in
  List.iter (fun x -> Printf.printf "%d " x) l;
  List.iter (fun x -> Printf.printf "%d " x) r;
