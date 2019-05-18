require "test_helper"

class CorundumTest < Minitest::Test
  def test_that_it_has_a_version_number
    refute_nil ::Corundum::VERSION
  end

  # def test_preview
  #   Corundum.preview(self, :empty).is_a?(String)
  # end
  #
  def test_run
    assert_nil Corundum.run(self, :empty)
  end
  #
  def test_run_variable
    assert_equal 1, Corundum.run(self, :variable)
  end
  #
  # def test_run_while_loop
  #   assert_equal 3_000_000, Corundum.run(self, :while_loop)
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

  def variable
    a = 0
    a
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
