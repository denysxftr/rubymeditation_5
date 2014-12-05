require 'benchmark'

require './ruby_fib'
require './librust_fib'


Benchmark.bmbm do |x|
  x.report("rust") { RustImplementation::fib(35) }

  x.report("ruby") { RubyImplementation::fib(35) }
end
