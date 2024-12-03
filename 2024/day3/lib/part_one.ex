defmodule PartOne do
  def start() do
    File.read!("input") |> sum_mul_instractions() |> IO.inspect(label: "part 1")
  end

  def sum_mul_instractions(instraction) do
    instractions(instraction)
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

  defp instractions(<<>>) do
    0
  end

  defp instractions(<<109, 117, 108, 40, first_num::24, 44, second_num::24, 41, rest::binary>>) do
    binary_to_int_24(first_num) * binary_to_int_24(second_num) + instractions(rest)
  end

  defp instractions(<<109, 117, 108, 40, first_num::24, 44, second_num::16, 41, rest::binary>>) do
    binary_to_int_24(first_num) * binary_to_int_16(second_num) + instractions(rest)
  end

  defp instractions(<<109, 117, 108, 40, first_num::24, 44, second_num::8, 41, rest::binary>>) do
    binary_to_int_24(first_num) * binary_to_int_8(second_num) + instractions(rest)
  end

  defp instractions(<<109, 117, 108, 40, first_num::16, 44, second_num::24, 41, rest::binary>>) do
    binary_to_int_16(first_num) * binary_to_int_24(second_num) + instractions(rest)
  end

  defp instractions(<<109, 117, 108, 40, first_num::16, 44, second_num::16, 41, rest::binary>>) do
    binary_to_int_16(first_num) * binary_to_int_16(second_num) + instractions(rest)
  end

  defp instractions(<<109, 117, 108, 40, first_num::16, 44, second_num::8, 41, rest::binary>>) do
    binary_to_int_16(first_num) * binary_to_int_8(second_num) + instractions(rest)
  end

  defp instractions(<<109, 117, 108, 40, first_num::8, 44, second_num::24, 41, rest::binary>>) do
    binary_to_int_8(first_num) * binary_to_int_24(second_num) + instractions(rest)
  end

  defp instractions(<<109, 117, 108, 40, first_num::8, 44, second_num::16, 41, rest::binary>>) do
    binary_to_int_8(first_num) * binary_to_int_16(second_num) + instractions(rest)
  end

  defp instractions(<<109, 117, 108, 40, first_num::8, 44, second_num::8, 41, rest::binary>>) do
    binary_to_int_8(first_num) * binary_to_int_8(second_num) + instractions(rest)
  end

  defp instractions(<<_, rest::binary>>) do
    instractions(rest)
  end
end
