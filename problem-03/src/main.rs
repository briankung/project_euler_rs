// The prime factors of 13195 are 5, 7, 13 and 29.
//
// What is the largest prime factor of the number 600_851_475_143 ?

fn main() {
    let target: i64 = 600_851_475_143;
    // println!("{:?}", (target as f64).sqrt() as i64);
    println!("{:?}", greatest_prime_factor(target));
}

fn is_factor(candidate: i64, target: i64) -> bool {
    target % candidate == 0
}

fn are_factors(candidates: Vec<i64>, target: i64) -> bool {
    target == candidates.iter().product()
}

fn nth_prime_sieve(n: usize) -> Vec<i64> {
    let mut primes: Vec<i64> = vec![2];
    let mut next_candidate = *primes.last().unwrap();

    'candidates: while primes.len() < n {
        next_candidate += 1;
        for prime in primes.iter() {
            if next_candidate % prime == 0 {
                continue 'candidates;
            }
        }

        primes.push(next_candidate);
    }

    primes
}

fn prime_lt(limit: i64) -> i64 {
    let mut primes: Vec<i64> = vec![2];
    let mut next_candidate = *primes.last().unwrap();

    'candidates: while primes.last().unwrap() < &limit {
        next_candidate += 1;

        if next_candidate >= limit {
            break;
        }

        for prime in primes.iter() {
            if next_candidate % prime == 0 {
                continue 'candidates;
            }
        }

        primes.push(next_candidate);
    }

    primes[primes.len() - 1]
}

fn greatest_prime_factor(target: i64) -> i64 {
    let mut primes: Vec<i64> = vec![2];
    let mut next_candidate = *primes.last().unwrap();
    let limit = (target as f64).sqrt() as i64;

    'candidates: while primes.last().unwrap() < &limit {
        next_candidate += 1;

        if target % next_candidate != 0 {
            continue;
        } else if next_candidate >= limit {
            break;
        }

        for prime in primes.iter() {
            if next_candidate % prime == 0 {
                continue 'candidates;
            }
        }

        println!("DEBUG {}\n", next_candidate);
        primes.push(next_candidate);
    }

    primes[primes.len() - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_factor_returns_true_for_factor() {
        assert!(is_factor(2, 4));
    }

    #[test]
    fn is_factor_returns_false_for_non_factor() {
        assert_eq!(is_factor(3, 4), false);
    }

    #[test]
    fn are_factors_returns_true_for_factors() {
        assert!(are_factors(vec![1, 2, 3], 6));
    }

    #[test]
    fn are_factors_returns_false_for_non_factors() {
        assert_eq!(are_factors(vec![1, 2, 4], 6), false);
    }
}

#[cfg(test)]
mod nth_prime_sieve_tests {
    use super::*;

    #[test]
    fn nth_prime_sieve_returns_n_amount_of_items() {
        assert!(nth_prime_sieve(3).len() == 3);
    }

    #[test]
    fn nth_prime_sieve_returns_n_amount_of_prime_numbers() {
        assert!(nth_prime_sieve(3) == vec![2, 3, 5]);
        assert!(nth_prime_sieve(4) == vec![2, 3, 5, 7]);
        assert!(nth_prime_sieve(5) == vec![2, 3, 5, 7, 11]);
    }
}

#[cfg(test)]
mod prime_lt_tests {
    use super::*;

    #[test]
    fn prime_lt_returns_largest_prime() {
        assert!(prime_lt(3) == 2);
        assert!(prime_lt(5) == 3);
        assert!(prime_lt(6) == 5);
        assert!(prime_lt(7) == 5);
        assert!(prime_lt(8) == 7);
    }

    // #[test]
    // fn prime_lt_returns_n_amount_of_prime_numbers() {
    //     assert!(prime_lt(3) == vec![2, 3, 5]);
    //     assert!(prime_lt(4) == vec![2, 3, 5, 7]);
    //     assert!(prime_lt(5) == vec![2, 3, 5, 7, 11]);
    // }
}
