use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: Sieve.exe <number limit>");
        return;
    }
    
    let _limit = args[1].parse::<usize>().unwrap();

    println!("Primes to {}: \n {:?}", _limit, sieve(_limit));
}
fn sieve(upper_limit: usize) -> Vec<usize> {
    let mut _prime_candidates = vec![true; upper_limit + 1];
    let mut _primes: Vec<usize> = Vec::new();


    for prime_candidate in 2..upper_limit {
        if !_prime_candidates[prime_candidate] {
            continue;
        }
        
        _primes.push(prime_candidate);

        if prime_candidate * prime_candidate >= upper_limit {
            continue;
        }
        
        let mut prime_candidate_multiple = prime_candidate + prime_candidate;
        while prime_candidate_multiple < upper_limit {
            _prime_candidates[prime_candidate_multiple] = false;
            prime_candidate_multiple += prime_candidate;
        }
    }
 
    return _primes;
}

#[test]
fn primes_to_ten() {
    let _primes = sieve(10);
    println!("Primes: {:?}", _primes);
    assert!(_primes == vec![2,3,5,7]);
}

#[test]
fn primes_to_hundred() {
    let _primes = sieve(100);
    println!("Primes: {:?}", _primes);
    assert!(_primes == vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97]);
}

#[test]
fn primes_to_thousand() {
    let _primes = sieve(1000);
    let _last_three_primes = &_primes[(_primes.len() - 3).._primes.len()];
    println!("Primes: {:?}", _last_three_primes);
    assert!(_last_three_primes == vec![983, 991, 997].as_slice());
}

#[test]
fn primes_to_tenthousand() {
    let _primes = sieve(10000);
    let _last_three_primes = &_primes[(_primes.len() - 3).._primes.len()];
    println!("Primes: {:?}", _last_three_primes);
    assert!(_last_three_primes == vec![9949, 9967, 9973].as_slice());
}