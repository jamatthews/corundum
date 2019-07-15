require 'benchmark'
require 'corundum'

def while_loop
  i = 0
  while i<30_000_000 # benchmark loop 1
    i += 1
  end
end

Benchmark.bmbm do |x|
  x.report("vm") { while_loop }
  x.report("baseline") { Corundum.run(self, :while_loop) }
  x.report("tracelet") { Corundum.run_tracelet(self, :while_loop) }
end
