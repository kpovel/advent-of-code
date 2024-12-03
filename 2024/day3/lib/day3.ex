defmodule Day3 do
  use Application

  def start(_, _) do
    PartOne.start()
    PartTwo.start()
    {:ok, self()}
  end
end
