require "helix_runtime"

begin
  require "corundum/native"
rescue LoadError
  warn "Unable to load corundum/native. Please run `rake build`"
end

class Corundum
  VERSION = "0.1.0"
end
