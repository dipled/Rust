pub fn fib(n: u32) {
    if n == 0 {
        return;
    }

    println!("0");

    if n == 1 {
        return;
    }

    let mut a: u32 = 0;
    let mut b: u32 = 1;

    println!("1");

    for _ in 2..n {
       let c = a + b;
       println!("{}", c);
       a = b;
       b = c
    }
}
