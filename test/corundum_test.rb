require "test_helper"

class CorundumTest < Minitest::Test
  def test_that_it_has_a_version_number
    refute_nil ::Corundum::VERSION
  end

  def test_preview
    Corundum.preview(self, :empty).is_a?(String)
  end

  def test_run
    assert_nil Corundum.run(self, :empty)
  end

  def test_run_variable_nil
    assert_equal nil, Corundum.run(self, :variable_nil)
  end

  def test_run_variable_zero
    assert_equal 0, Corundum.run(self, :variable_zero)
  end

  def test_run_plus
    assert_equal 2, Corundum.run(self, :plus)
  end

  def test_run_plus
    assert_equal 2, Corundum.run(self, :plus)
  end

  def test_run_plus
    assert_equal 2, Corundum.run(self, :plus)
  end

  def test_run_less_than
    assert_equal true, Corundum.run(self, :less_than)
  end

  def test_run_test_if
    assert_equal 0, Corundum.run(self, :test_if)
  end

  # def test_run_while_loop
  #   assert_equal 3_000_000, Corundum.preview(self, :while_loop)
  # end

  #
  # def test_argument
  #   assert_equal 2, Corundum.run(self, :argument, [1])
  # end

  # def test_preview_iseqw
  #   Corundum.preview_iseqw(self, :empty)
  # end

  # def test_disasm_iseqw
  #   method = self.method(:variable)
  #   iseqw = RubyVM::InstructionSequence.of(method)
  #   puts iseqw.disasm
  # end

  private

  def empty
  end

  def variable_nil
    a = nil
    a
  end

  def variable_zero
    a = 0
    a
  end

  def plus
    a = 1 + 1
    a
  end

  def less_than
    0 < 1
  end

  def test_if
      0 if 0 < 1
  end

  def while_loop
    i = 0
    while i < 3_000_000 do
      i = i + 1
    end
    i
  end

  def argument(x)
    x + 1
  end
end
