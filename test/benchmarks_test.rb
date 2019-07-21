require "test_helper"

class BenchmarksTest < Minitest::Test

  def test_while_loop
    assert_equal 3_000_000, Corundum.compile_and_run(self, method(:while_loop))
  end

  # def test_factorial
  #   assert_equal 1, Corundum.run(self, :factorial, 1)
  # end

  private

  def while_loop
    i = 0
    while i < 3_000_000 do
      i = i + 1
    end
    i
  end

  def factorial(n = 1)
    if(n > 1)
      n * factorial(n-1)
    else
      1
    end
  end
end
