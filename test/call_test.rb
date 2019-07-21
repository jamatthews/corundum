require "test_helper"

class CallTest < Minitest::Test

  # def test_call
  #   assert_equal 1, Corundum.run(self, :caller)
  # end

  private

  def caller
    callee
  end

  def self.callee
    1
  end


end
