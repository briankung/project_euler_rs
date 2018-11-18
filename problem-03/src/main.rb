target = 600_851_475_143

def prime_factor_lt(limit, target)
  primes = [2]
  next_candidate = 2

  while primes.last < limit
    next_candidate += 1

    if next_candidate >= limit
      break
    elsif target % next_candidate != 0
      next
    end

    next if primes.any? {|prime| next_candidate % prime == 0}

    puts "DEBUG #{next_candidate}"
    primes << next_candidate
  end

  primes.last
end

puts prime_factor_lt(Math.sqrt(target), target)
