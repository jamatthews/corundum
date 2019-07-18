require "helix_runtime"

begin
  require "corundum/native"
rescue LoadError
  warn "Unable to load corundum/native. Please run `rake build`"
end

class Corundum

  VERSION = "0.1.0"

  def self.preview(receiver, name, args = [])
    method = receiver.method(name)
    preview_cranelift_ir(receiver, method)
  end

  def self.compile_only(receiver, name, args = [])
    method = receiver.method(name)
    iseqw = RubyVM::InstructionSequence.of(method)
    return false if iseqw.nil?
    compile("#{receiver.class.name}#{name}", iseqw)
  end

  def self.compile_tracelet_only(receiver, name, args = [])
    method = receiver.method(name)
    iseqw = RubyVM::InstructionSequence.of(method)
    return false if iseqw.nil?
    compile_tracelet("#{receiver.class.name}#{name}", iseqw)
  end

  def self.run(receiver, name, args = [])
    method = receiver.method(name)
    iseqw = RubyVM::InstructionSequence.of(method)
    return false if iseqw.nil?
    compile_and_run("#{receiver.class.name}#{name}", iseqw)
  end

  def self.run_tracelet(receiver, name, args = [])
    method = receiver.method(name)
    iseqw = RubyVM::InstructionSequence.of(method)
    return false if iseqw.nil?
    compile_and_run_tracelet("#{receiver.class.name}#{name}", iseqw)
  end
end
