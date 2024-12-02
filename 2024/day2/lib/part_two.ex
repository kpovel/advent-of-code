defmodule PartTwo do
  def start() do
    # this part isn't right

    File.stream!("input")
    |> Stream.map(&String.trim/1)
    |> Stream.map(fn record ->
      String.split(record, " ")
      |> Enum.map(&String.to_integer/1)
    end)
    |> Stream.map(&save_record?/1)
    |> Enum.to_list()
    |> Enum.filter(fn save -> save == true end)
    |> Enum.count()
    |> IO.inspect(label: "part 2")
  end

  def save_record?(record) do
    save_record?(record, 0, nil)
  end

  defp save_record?(_, bad_reports, _) when bad_reports > 1 do
    false
  end

  defp save_record?([], bad_reports, _) do
    bad_reports <= 1
  end

  defp save_record?([_hd], bad_reports, _) do
    save_record?([], bad_reports, nil)
  end

  defp save_record?([hd, hd2 | tl], bad_reports, :increasing) do
    case hd2 - hd do
      diff when diff in 1..3 ->
        save_record?([hd2 | tl], bad_reports, :increasing)

      _ ->
        save_record?([hd | tl], bad_reports + 1, :increasing)
    end
  end

  defp save_record?([hd, hd2 | tl], bad_reports, :decreasing) do
    case hd - hd2 do
      diff when diff in 1..3 ->
        save_record?([hd2 | tl], bad_reports, :decreasing)

      _ ->
        save_record?([hd | tl], bad_reports + 1, :decreasing)
    end
  end

  defp save_record?([_hd, hd2 | tl] = record, bad_reports, nil) do
    save_record?(record, bad_reports, :decreasing) or
      save_record?([hd2 | tl], bad_reports + 1, :decreasing) or
      save_record?(record, bad_reports, :increasing) or
      save_record?([hd2 | tl], bad_reports + 1, :increasing)
  end
end
