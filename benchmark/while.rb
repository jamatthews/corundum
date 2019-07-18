require 'benchmark'

$LOAD_PATH.unshift File.expand_path("../../lib", __FILE__)
require 'corundum'

def while_loop
  i = 0
  while i < 20_000 # benchmark loop 1
    i += 1
  end
end

Benchmark.bmbm do |x|
  x.report("vm") { while_loop }
  x.report("baseline compile") { Corundum.compile_only(self, :while_loop) }
  x.report("baseline compile and run") { Corundum.run(self, :while_loop) }
  x.report("tracelet compile only") { Corundum.compile_tracelet_only(self, :while_loop) }
  x.report("tracelet compile and run") { Corundum.run_tracelet(self, :while_loop) }
end
