require "test_helper"

class CorundumTest < Minitest::Test
  def test_that_it_has_a_version_number
    refute_nil ::Corundum::VERSION
  end

  def test_preview
    Corundum.preview(self, :empty)
  end

  def test_while_loop
    Corundum.preview(self, :while_loop)
  end

  def test_preview
    Corundum.run(self, :empty)
  end

  def test_while_loop
    Corundum.run(self, :while_loop)
  end

  private
  def empty
  end

  def while_loop
    i = 0
    while i < 3_000_000 do
      i = i + 1
    end
  end
end
