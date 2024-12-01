defmodule Day1 do
  use Application

  @impl true
  def start(_, _) do
    PartOne.start()
    PartTwo.start()

    {:ok, self()}
  end
end
