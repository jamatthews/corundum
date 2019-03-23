require "helix_runtime"

begin
  require "corundum/native"
rescue LoadError
  warn "Unable to load corundum/native. Please run `rake build`"
end

class Corundum

  VERSION = "0.1.0"

  def self.preview(recveiver, name)
    method = recveiver.method(name)
    iseq = RubyVM::InstructionSequence.of(method)
    return false if iseq.nil?

    puts "#{iseq.disasm}\n"
    preview_cranelift_ir(['putnil','leave'])
  end
end
