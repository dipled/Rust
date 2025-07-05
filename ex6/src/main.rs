use std::ops::Range;

fn is_prime(x : u128) -> bool
{
    if x < 4 {
        if x == 1 || x == 0 {return false;}
        if x == 2 || x == 3 {return true;}
    }
    if x % 2 == 0 {return false;}
    let mut i : u128 = 3;
    loop {
        if x % i == 0 {return false;}
        if i * i >= x {return true;}
        i += 2;
    }
}

fn list_primes(start: u128, end : u128) -> Vec<u128>
{
    let range : Range<u128> = start .. end;
    return range.filter(|x| is_prime(*x)).collect();
}

fn main() {
    println!("{:?}", list_primes(0, 100))
}
