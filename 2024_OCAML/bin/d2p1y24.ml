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

let () =
  let lines = read_file_lines "input.txt" in
  let nums = List.map (fun el -> split_and_extract_numbers el) lines in
  List.iter (fun el -> print_int_list el) nums
;;
