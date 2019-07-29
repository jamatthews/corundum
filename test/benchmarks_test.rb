require "test_helper"

class BenchmarksTest < Minitest::Test

  def test_while_loop
    assert_equal 3_000_000, Corundum.compile_and_run(self, method(:while_loop))
  end

  def test_factorial
    assert_equal 6, Corundum.compile_and_run(self, BenchmarksTest.method(:factorial))
  end

  def test_array
    assert_equal [], Corundum.compile_and_run(self, method(:array))
  end

  private

  def while_loop
    i = 0
    while i < 3_000_000 do
      i = i + 1
    end
    i
  end

  def self.factorial
    fact(3)
  end

  def array
    Array.new(0)
  end
end
