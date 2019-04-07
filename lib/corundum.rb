require "helix_runtime"

begin
  require "corundum/native"
rescue LoadError
  warn "Unable to load corundum/native. Please run `rake build`"
end

class Corundum

  VERSION = "0.1.0"

  def self.preview(receiver, name)
    method = receiver.method(name)
    iseq = RubyVM::InstructionSequence.of(method)
    return false if iseq.nil?
    preview_cranelift_ir("#{receiver.class.name}#{name}", ['putnil','leave'])
  end
end
