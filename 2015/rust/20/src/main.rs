use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

type Input = u64;

fn parse_input(input: &str) -> Input {
    input.trim().parse().unwrap()
}

fn is_prime(primes: &mut (Vec<u64>, Vec<bool>), n: u64) -> bool {
    let square_root = (n as f64).sqrt().floor() as u64;

    for prime in &primes.0 {
        if n % *prime == 0 {
            primes.1.push(false);
            return false;
        }
        if *prime > square_root {
            break;
        }
    }

    primes.0.push(n);
    primes.1.push(true);
    return true;
}

fn factorize(primes: &mut (Vec<u64>, Vec<bool>), mut n: u64) -> Vec<(u64, u64)> {
    if is_prime(primes, n) {
        return vec![(n, 1)];
    }

    let mut factors = vec![];

    let mut square_root = (n as f64).sqrt().floor() as u64;

    for prime in &primes.0 {
        if *prime > square_root {
            if primes.1[n as usize] {
                factors.push((n, 1));
                return factors;
            }
        }

        if n % *prime == 0 {
            n /= *prime;
            let mut multiplicity = 1u64;
            while n % *prime == 0 {
                n /= *prime;
                multiplicity += 1;
            }
            factors.push((*prime, multiplicity));
            if n == 1 {
                return factors;
            }

            square_root = (n as f64).sqrt().floor() as u64;
        }
    }

    unreachable!()
}

fn task1(input: &Input) -> u64 {
    let mut primes = (vec![], vec![false, false]);
    for i in 2.. {
        let factors = factorize(&mut primes, i);

        let sum_of_factors = factors.iter().fold(1, |acc, (prime, multiplicity)| {
            acc * (prime.pow(*multiplicity as u32 + 1) - 1) / (prime - 1)
        });

        if sum_of_factors * 10 > *input {
            return i;
        }
    }

    unreachable!()
}

fn task2(input: &Input) -> u64 {
    let mut primes = (vec![], vec![false, false]);
    for i in 2.. {
        let factors = factorize(&mut primes, i);

        fn find_all_factors(
            remaining_factors: &[(u64, u64)],
            current_product: u64,
            min_product: u64,
            sum: &mut u64,
        ) {
            if remaining_factors.len() == 0 {
                if current_product >= min_product {
                    *sum += current_product;
                }
                return;
            }

            let (factor, multiplicity) = remaining_factors[0];
            for n in 0..=multiplicity {
                find_all_factors(
                    &remaining_factors[1..],
                    current_product * factor.pow(n as u32),
                    min_product,
                    sum,
                );
            }
        }

        let min_product = (i + 49) / 50;
        let mut sum_of_factors = 0;
        find_all_factors(&factors[..], 1, min_product, &mut sum_of_factors);

        if sum_of_factors * 11 > *input {
            return i;
        }
    }

    unreachable!()
}

fn main() {
    let input = parse_input(&load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input), 831600);
    assert_eq!(task2(&input), 884520);
}
