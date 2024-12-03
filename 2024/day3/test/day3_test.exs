defmodule Day3Test do
  use ExUnit.Case
  doctest Day3

  @instraction "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
  @part_two "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"

  test "part 1" do
    assert PartOne.sum_mul_instractions(@instraction) == 161
  end

  test "part 2" do
    assert PartTwo.sum_mul_instractions(@part_two) == 48
  end
end
