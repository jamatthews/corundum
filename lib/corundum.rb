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
    iseq = RubyVM::InstructionSequence.of(method)
    return false if iseq.nil?
    preview_cranelift_ir("#{receiver.class.name}#{name}", stringify_iseq(iseq), args)
  end

  def self.run(receiver, name, args = [])
    method = receiver.method(name)
    iseq = RubyVM::InstructionSequence.of(method)
    return false if iseq.nil?
    compile_and_run("#{receiver.class.name}#{name}", stringify_iseq(iseq), args)
  end

  def self.preview_iseqw(receiver, name)
    method = receiver.method(:empty)
    iseqw = RubyVM::InstructionSequence.of(method)
    print_iseqw(iseqw)
  end

  private

  def self.stringify_iseq(iseq)
    iseq.to_a.last
      .select{|x| x.is_a?(Symbol) || x.is_a?(Array) } #strip out extra stuff
      .reject{|x| x.is_a?(Array) && x.first == :trace } #strip out extra stuff
      .map!{|x| if x.is_a?(Symbol); x.to_s.split('_') ; else x; end } #split label instructions
      .map!{|x| if x.is_a?(Array); x.map(&:to_s) ; else x; end } #instructions are arrays, need to make elements strings
      .map!{|x| if x.is_a?(Array) && ['jump','branchif'].include?(x[0]) ; x[1].gsub!('label_','') ; end; x } #remove label prefix
  end
end
