defmodule PartOne do
  def start() do
    File.stream!("input")
    |> Stream.map(&String.trim/1)
    |> Enum.reduce([[], []], fn line, [group1, group2] ->
      [location1, location2] = String.split(line, "   ")
      location1 = String.to_integer(location1)
      location2 = String.to_integer(location2)

      group1 = [location1 | group1]
      group2 = [location2 | group2]
      [group1, group2]
    end)
    |> Enum.map(&Enum.sort/1)
    |> total_distances()
    |> IO.inspect(label: "part 1")
  end

  defp total_distances([[], []]), do: 0

  defp total_distances([[hd1 | group1], [hd2 | group2]]) do
    abs(hd2 - hd1) + total_distances([group1, group2])
  end
end
