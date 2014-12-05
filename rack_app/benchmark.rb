require './librack_app_rust'
require 'benchmark'

module RubyRack
  def self.call(env)
    [200, {}, ["Hello world!"]]
  end
end

Benchmark.bmbm do |x|
  x.report("rust") do
    1_000_000.times { RackAppRust.call({a: 5}) }
  end

  x.report("ruby") do
    1_000_000.times { RubyRack.call({a: 5}) }
  end
end
