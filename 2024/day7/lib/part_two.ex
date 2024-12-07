defmodule PartTwo do
  def main() do
    File.read!("input")
    |> parse_input()
    |> Enum.map(fn input ->
      valid_calculation?(input)
    end)
    |> Enum.sum()
  end

  def valid_calculation?({expected, calculation}) do
    Enum.count(calculation)
    |> possible_operators()
    |> Enum.map(fn operators ->
      valid?({expected, calculation}, operators)
    end)
    |> Enum.any?()
    |> case do
      true -> expected
      false -> 0
    end
  end

  defp valid?({expected, [hd, hd2]}, [hd_operator]) do
    res =
      case hd_operator do
        :multiply ->
          hd * hd2

        :add ->
          hd + hd2

        :combine ->
          (Integer.to_string(hd) <> Integer.to_string(hd2)) |> String.to_integer()
      end

    res == expected
  end

  defp valid?({expected, [hd, hd2 | calculations]}, [hd_operator | tl_operator]) do
    case hd_operator do
      :multiply ->
        valid?({expected, [hd * hd2 | calculations]}, tl_operator)

      :add ->
        valid?({expected, [hd + hd2 | calculations]}, tl_operator)

      :combine ->
        sum = (Integer.to_string(hd) <> Integer.to_string(hd2)) |> String.to_integer()
        valid?({expected, [sum | calculations]}, tl_operator)
    end
  end

  def parse_input(input) do
    String.split(input, "\n", trim: true)
    |> Enum.map(fn calibration ->
      [calibration, values] = String.splitter(calibration, [":", ","]) |> Enum.to_list()
      values = String.split(values, " ", trim: true) |> Enum.map(&String.to_integer/1)

      {String.to_integer(calibration), values}
    end)
  end

  def possible_operators(n) when n > 0 do
    for combination <-
          Enum.reduce(1..(n - 1), [[]], fn _, acc ->
            for combo <- acc, operator <- [:multiply, :add, :combine] do
              [operator | combo]
            end
          end) do
      Enum.reverse(combination)
    end
  end
end
