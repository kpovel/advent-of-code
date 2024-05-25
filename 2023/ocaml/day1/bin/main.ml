open Base

let explode_string s = List.init (String.length s) ~f:(String.get s)
let read_lines file = In_channel.with_open_bin file In_channel.input_all |> String.strip

let part1 input =
  String.split_lines input
  |> List.map ~f:(fun acc ->
    let chars = explode_string acc in
    let first =
      List.find_exn chars ~f:(fun x ->
        match x with
        | '0' .. '9' -> true
        | _ -> false)
      |> Char.to_string
      |> Int.of_string
    in
    let last =
      List.rev chars
      |> List.find_exn ~f:(fun x ->
        match x with
        | '0' .. '9' -> true
        | _ -> false)
      |> Char.to_string
      |> Int.of_string
    in
    (first * 10) + last)
  |> List.fold ~init:0 ~f:(fun acc x -> acc + x)
;;

let part2 input =
  let rec get_num l =
    match l with
    | 'o' :: 'n' :: 'e' :: _ | '1' :: _ -> 1
    | 't' :: 'w' :: 'o' :: _ | '2' :: _ -> 2
    | 't' :: 'h' :: 'r' :: 'e' :: 'e' :: _ | '3' :: _ -> 3
    | 'f' :: 'o' :: 'u' :: 'r' :: _ | '4' :: _ -> 4
    | 'f' :: 'i' :: 'v' :: 'e' :: _ | '5' :: _ -> 5
    | 's' :: 'i' :: 'x' :: _ | '6' :: _ -> 6
    | 's' :: 'e' :: 'v' :: 'e' :: 'n' :: _ | '7' :: _ -> 7
    | 'e' :: 'i' :: 'g' :: 'h' :: 't' :: _ | '8' :: _ -> 8
    | 'n' :: 'i' :: 'n' :: 'e' :: _ | '9' :: _ -> 9
    | _ :: tl -> get_num tl
    | _ -> raise (Invalid_argument "")
  in
  let rec get_num_rev l =
    match l with
    | 'e' :: 'n' :: 'o' :: _ | '1' :: _ -> 1
    | 'o' :: 'w' :: 't' :: _ | '2' :: _ -> 2
    | 'e' :: 'e' :: 'r' :: 'h' :: 't' :: _ | '3' :: _ -> 3
    | 'r' :: 'u' :: 'o' :: 'f' :: _ | '4' :: _ -> 4
    | 'e' :: 'v' :: 'i' :: 'f' :: _ | '5' :: _ -> 5
    | 'x' :: 'i' :: 's' :: _ | '6' :: _ -> 6
    | 'n' :: 'e' :: 'v' :: 'e' :: 's' :: _ | '7' :: _ -> 7
    | 't' :: 'h' :: 'g' :: 'i' :: 'e' :: _ | '8' :: _ -> 8
    | 'e' :: 'n' :: 'i' :: 'n' :: _ | '9' :: _ -> 9
    | _ :: tl -> get_num_rev tl
    | _ -> raise (Invalid_argument "")
  in
  String.split_lines input
  |> List.map ~f:(fun x ->
    let chars = explode_string x in
    let first = get_num chars in
    let last = List.rev chars |> get_num_rev in
    (first * 10) + last)
  |> List.fold ~init:0 ~f:(fun acc x -> acc + x)
;;

let () =
  read_lines "input" |> part1 |> Stdlib.Printf.printf "First part: %d\n";
  read_lines "input" |> part2 |> Stdlib.Printf.printf "Second part: %d\n"
;;
