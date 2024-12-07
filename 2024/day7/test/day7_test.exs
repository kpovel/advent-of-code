defmodule Day7Test do
  use ExUnit.Case
  doctest Day7

  test "part one" do
    assert PartOne.possible_operators(2) == [[:multiply], [:add]]

    assert PartOne.possible_operators(3) == [
             [:multiply, :multiply],
             [:multiply, :add],
             [:add, :multiply],
             [:add, :add]
           ]
  end
end
