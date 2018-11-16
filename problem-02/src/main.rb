current_fibs = [1,2]
max_size = 4_000_000

loop do
  next_fib = current_fibs.values_at(-1, -2).sum
  break if next_fib > max_size
  current_fibs << next_fib
end

puts current_fibs.select(&:even?).sum
