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

let split_and_extract_numbers s =
  let regex = Str.regexp "[ \t]+" in
  let parts = Str.split regex s in
  List.map int_of_string parts

let count_occurrences lst x =
  let rec aux l acc =
    match l with
    | n :: rest -> if n = x then aux rest (acc + 1) else aux rest acc
    | [] -> acc + 0 in
  aux lst 0

let sum_scores l1 l2 =
  let rec aux l1 l2 acc =
    match l1 with
    | n :: rest -> aux rest l2 (acc + (n * (count_occurrences l2 n)))
    | [] -> acc in
  aux l1 l2 0

let () =
  let lines = read_file_lines "input.txt" in
  let pairs = List.map (fun el -> split_and_extract_numbers el) lines in
  let (l1, l2) = List.fold_left (fun (lst1, lst2) l ->
        match l with
        | [x; y] -> (x :: lst1, y :: lst2)
        | _ -> (lst1, lst2)
      ) ([], []) pairs in
  let sum = sum_scores (List.sort compare l1) (List.sort compare l2) in
  Printf.printf "\nSUM: %d\n" sum
