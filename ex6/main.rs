use std::ops::Range;
use std::thread;
use std::env;

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
    (start..=end).(|x : &u128| is_prime(*x)).collect();
}

fn main() {
    let n : u128 = env::args().nth(1).unwrap().parse().expect("provide the upper limit for the interval");
    let num_threads : u128 = env::args().nth(2).unwrap().parse().expect("provide the number of threads");
    let mut handler  = Vec::new();

    let mut s = 0;
    for i in 1 .. num_threads + 1 {
        handler.push (thread::spawn(move || list_primes(s, i * n / num_threads)));
        s = i * n / num_threads + 1;
    }

    let mut results : Vec<u128> = Vec::new();

    for handle in handler {
        results.append(&mut handle.join().unwrap());
    }

}
