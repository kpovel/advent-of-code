defmodule Day2Test do
  use ExUnit.Case
  doctest Day2

  test "part 2" do
    assert PartTwo.save_record?([7, 6, 4, 2, 1])
    assert not PartTwo.save_record?([7, 6, 4, 2, 9, 17])
    assert PartTwo.save_record?([7, 6, 4, 2, 1])
    assert not PartTwo.save_record?([1, 2, 7, 8, 9])
    assert not PartTwo.save_record?([9, 7, 6, 2, 1])
    assert PartTwo.save_record?([1, 3, 2, 4, 5])
    assert PartTwo.save_record?([8, 6, 4, 4, 1])
    assert PartTwo.save_record?([1, 3, 6, 7, 9])
  end
end
