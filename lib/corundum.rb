require "helix_runtime"
require "pp"

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
    stringified_iseq = iseq.to_a.last
      .select{|x| x.is_a?(Symbol) || x.is_a?(Array) } #strip out extra stuff
      .reject{|x| x.is_a?(Array) && x.first == :trace }
      .map!{|x| if x.is_a?(Symbol); x.to_s.split('_') ; else x; end }
      .map!{|x| if x.is_a?(Array); x.map(&:to_s) ; else x; end }
      .map!{|x| if x.is_a?(Array) && ['jump','branchif'].include?(x[0]) ; x[1].gsub!('label_','') ; end; x }
    preview_cranelift_ir("#{receiver.class.name}#{name}", stringified_iseq)
  end
end
