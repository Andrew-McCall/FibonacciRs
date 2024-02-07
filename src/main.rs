use std::time::Instant;
use num_bigint::{BigUint};

fn fibinacci(n: u64) -> u64 {
    if n == 0 || n == 1 {
        return n;
    }
    return fibinacci(n - 1) + fibinacci(n - 2);
}

fn fibinacci_iterative(n: u64) -> BigUint {
    let mut a: BigUint = 0u64.into();
    let mut b: BigUint = 1u64.into();

    for _ in 0..n {
        let tmp = a.clone();
        a = b.clone();
        b += tmp;
    }
    a
}

fn main() {

    println!("Enter a number to find its fibinacci");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    match input.trim().parse::<u64>() {
        Ok(n) => {
            let start = Instant::now();
            let result = fibinacci(n);
            let duration = start.elapsed();

            let startS = Instant::now();
            let resultS = fibinacci_iterative(n);
            let durationS = startS.elapsed();
            println!("Fibinacci of {} is and took {:?}", n, duration);
            println!("FibinacciS of {} is and took {:?}", n, durationS);
        },
        Err(_) => println!("Invalid input"),
    }

    main();
}
