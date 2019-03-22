require "test_helper"

class CorundumTest < Minitest::Test
  def test_that_it_has_a_version_number
    refute_nil ::Corundum::VERSION
  end

  def empty
  end

  def test_preview
    Corundum.preview(self, :empty)
  end
end
