require 'benchmark'

$LOAD_PATH.unshift File.expand_path("../../lib", __FILE__)
require 'corundum'

def while_loop
  i = 0
  while i < 30_000_000
    i += 1
  end
end

Benchmark.bmbm do |x|
  x.report("vm") { while_loop }
  x.report("baseline compile") { Corundum.compile(self, method(:while_loop)) }
  x.report("baseline compile and run") { Corundum.compile_and_run(self, method(:while_loop)) }
end
