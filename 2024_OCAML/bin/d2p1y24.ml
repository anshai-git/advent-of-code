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
;;

let split_and_extract_numbers s =
  let regex = Str.regexp " " in
  let parts = Str.split regex s in
  List.map int_of_string parts
;;

let print_int_list lst =
  List.iter (fun el -> Printf.printf "%d;" el) lst;
  Printf.printf "\n"
;;

let is_ascending report =
  match report with
  | first :: second :: _ -> first > second
  | _ -> false
;;

let verify_ascending report =
  let rec aux lst =
    match lst with
    | [] -> true
    | [e1 :: e2 :: rest] -> if e1 > e2 then false else aux [e2 :: res]
    | _ -> false
;;

let is_safe report =
  let is_asc = is_ascending report in
  verify_ascending report
;;

let () =
  let lines = read_file_lines "d2_ex1.txt" in
  let num_lists = List.map (fun el -> split_and_extract_numbers el) lines in

  (* List.iter (fun el -> print_int_list el) nums *)
;;
