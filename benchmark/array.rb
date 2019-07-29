n = 9000 # Integer(ARGV.shift || 1)

x = Array.new(n)
y = Array.new(n, 0)

n.times{|bi|
  x[bi] = bi + 1
}

(0 .. 999).each do |e|
  (n-1).step(0,-1) do |bi|
    y[bi] += x.at(bi)
  end
end
