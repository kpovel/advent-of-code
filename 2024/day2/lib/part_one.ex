defmodule PartOneServer do
  use GenServer

  @impl true
  def init(_) do
    {:ok, 0}
  end

  def start() do
    GenServer.start(__MODULE__, nil, name: __MODULE__)
  end

  @impl true
  def handle_cast({:parse_report, report}, state) do
    case PartOne.save_record?(report) do
      true -> {:noreply, state + 1}
      _ -> {:noreply, state}
    end
  end

  @impl true
  def handle_call(:save_count, _, state) do
    {:reply, state, state}
  end

  def parse_report(report) do
    GenServer.cast(__MODULE__, {:parse_report, report})
  end

  def save_count() do
    GenServer.call(__MODULE__, :save_count)
  end
end

defmodule PartOne do
  def start() do
    PartOneServer.start()

    File.stream!("input")
    |> Stream.map(&String.trim/1)
    |> Enum.each(&PartOneServer.parse_report/1)

    PartOneServer.save_count() |> IO.inspect(label: "part 1")
  end

  def save_record?(record) do
    String.split(record, " ")
    |> Enum.map(&String.to_integer/1)
    |> save_record?(nil)
  end

  defp save_record?([hd | tl], prev, :increasing) do
    case prev - hd do
      diff when diff <= -1 and diff >= -3 -> save_record?(tl, hd, :increasing)
      _ -> false
    end
  end

  defp save_record?([hd | tl], prev, :decreasing) do
    case prev - hd do
      diff when diff >= 1 and diff <= 3 -> save_record?(tl, hd, :decreasing)
      _ -> false
    end
  end

  defp save_record?([], _prev, _) do
    true
  end

  defp save_record?([hd, hd2 | tl], nil) do
    case {hd, hd2} do
      {hd, hd2} when hd > hd2 ->

        save_record?([hd2 | tl], hd, :decreasing)

      {hd, hd2} when hd < hd2 ->
        save_record?([hd2 | tl], hd, :increasing)

      {hd, hd2} when hd == hd2 ->
        false
    end
  end
end
