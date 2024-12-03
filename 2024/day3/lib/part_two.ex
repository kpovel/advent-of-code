defmodule PartTwo do
  def start() do
    File.read!("input") |> sum_mul_instractions() |> IO.inspect(label: "part 2")
  end

  def sum_mul_instractions(instraction) do
    instractions(true, instraction)
  end

  defp binary_to_int_24(binary) do
    case <<binary::24>> |> to_string() |> Integer.parse() do
      {int, _} -> int
    end
  end

  defp binary_to_int_16(binary) do
    case <<binary::16>> |> to_string() |> Integer.parse() do
      {int, _} -> int
    end
  end

  defp binary_to_int_8(binary) do
    case <<binary::8>> |> to_string() |> Integer.parse() do
      {int, _} -> int
    end
  end

  defp instractions(_, <<>>) do
    0
  end

  defp instractions(_, <<100, 111, 110, 39, 116, 40, 41, rest::binary>>) do
    instractions(false, rest)
  end

  defp instractions(_, <<100, 111, 40, 41, rest::binary>>) do
    instractions(true, rest)
  end

  defp instractions(
         execute,
         <<109, 117, 108, 40, first_num::24, 44, second_num::24, 41, rest::binary>>
       ) do
    case execute do
      true ->
        binary_to_int_24(first_num) * binary_to_int_24(second_num) + instractions(execute, rest)

      false ->
        instractions(execute, rest)
    end
  end

  defp instractions(
         execute,
         <<109, 117, 108, 40, first_num::24, 44, second_num::16, 41, rest::binary>>
       ) do
    case execute do
      true ->
        binary_to_int_24(first_num) * binary_to_int_16(second_num) + instractions(execute, rest)

      false ->
        instractions(execute, rest)
    end
  end

  defp instractions(
         execute,
         <<109, 117, 108, 40, first_num::24, 44, second_num::8, 41, rest::binary>>
       ) do
    case execute do
      true ->
        binary_to_int_24(first_num) * binary_to_int_8(second_num) + instractions(execute, rest)

      false ->
        instractions(execute, rest)
    end
  end

  defp instractions(
         execute,
         <<109, 117, 108, 40, first_num::16, 44, second_num::24, 41, rest::binary>>
       ) do
    case execute do
      true ->
        binary_to_int_16(first_num) * binary_to_int_24(second_num) + instractions(execute, rest)

      false ->
        instractions(execute, rest)
    end
  end

  defp instractions(
         execute,
         <<109, 117, 108, 40, first_num::16, 44, second_num::16, 41, rest::binary>>
       ) do
    case execute do
      true ->
        binary_to_int_16(first_num) * binary_to_int_16(second_num) + instractions(execute, rest)

      false ->
        instractions(execute, rest)
    end
  end

  defp instractions(
         execute,
         <<109, 117, 108, 40, first_num::16, 44, second_num::8, 41, rest::binary>>
       ) do
    case execute do
      true ->
        binary_to_int_16(first_num) * binary_to_int_8(second_num) + instractions(execute, rest)

      false ->
        instractions(execute, rest)
    end
  end

  defp instractions(
         execute,
         <<109, 117, 108, 40, first_num::8, 44, second_num::24, 41, rest::binary>>
       ) do
    case execute do
      true ->
        binary_to_int_8(first_num) * binary_to_int_24(second_num) + instractions(execute, rest)

      false ->
        instractions(execute, rest)
    end
  end

  defp instractions(
         execute,
         <<109, 117, 108, 40, first_num::8, 44, second_num::16, 41, rest::binary>>
       ) do
    case execute do
      true ->
        binary_to_int_8(first_num) * binary_to_int_16(second_num) + instractions(execute, rest)

      false ->
        instractions(execute, rest)
    end
  end

  defp instractions(
         execute,
         <<109, 117, 108, 40, first_num::8, 44, second_num::8, 41, rest::binary>>
       ) do
    case execute do
      true ->
        binary_to_int_8(first_num) * binary_to_int_8(second_num) + instractions(execute, rest)

      false ->
        instractions(execute, rest)
    end
  end

  defp instractions(execute, <<_, rest::binary>>) do
    instractions(execute, rest)
  end
end
