require 'benchmark'

$LOAD_PATH.unshift File.expand_path("../../lib", __FILE__)
require 'corundum'

def factorial
  fact(30)
end

def fact(n)
  if(n > 1)
    n * fact(n-1)
  else
    1
  end
end

Benchmark.bmbm do |x|
  x.report("vm") { factorial }
  x.report("baseline compile") { Corundum.compile(self, method(:factorial)) }
  x.report("baseline compile and run ") { Corundum.compile_and_run(self, method(:factorial)) }
end
