defmodule PartTwo do
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
    |> similar_score([])
    |> Enum.map(fn {key, value} -> key * value end)
    |> Enum.sum()
    |> IO.inspect(label: "part 2")
  end

  defp similar_score([[], _], score), do: score

  defp similar_score([[hd1 | group1], group2], score) do
    Enum.find(score, fn {key, _} -> key == hd1 end)
    |> case do
      nil ->
        location_count =
          Enum.filter(group2, fn location -> location == hd1 end)
          |> Enum.count()

        similar_score([group1, group2], [{hd1, location_count} | score])

      val ->
        similar_score([group1, group2], [val | score])
    end
  end
end
