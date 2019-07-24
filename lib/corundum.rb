require "helix_runtime"

begin
  require "corundum/native"
rescue LoadError
  warn "Unable to load corundum/native. Please run `rake build`"
end

class Corundum
  VERSION = "0.1.0"

  # only here because I haven't implemented method lookup properly yet
  def self.fact(n)
    if(n > 1)
      n * fact(n-1)
    else
      1
    end
  end
end
