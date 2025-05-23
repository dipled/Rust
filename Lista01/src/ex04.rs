use rand::{rngs::ThreadRng, Rng};

pub fn is_prime_bruteforce(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..n - 1 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn mod_exp(mut base: u64, mut exp: u64, modu: u64) -> u64 {
    let mut result = 1;
    base %= modu;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modu;
        }
        exp >>= 1;
        base = (base * base) % modu;
    }
    result
}

pub fn is_prime_fermat(n: u64, iterations: u64) -> bool {
    if n <= 1 {
        return false;
    }

    if n == 2 {
        return true;
    }

    let mut rng: ThreadRng = rand::rng();

    for _ in 0..iterations {        
        let a: u64 = rng.random_range(2..=n - 2);
        if mod_exp(a, n - 1, n) != 1 {
            return false;
        }
    }
    true
}
