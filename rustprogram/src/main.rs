use std::time::SystemTime;
extern crate num;
use num::{BigInt, One};

fn factorial(n: u64) -> BigInt {
    let mut result = BigInt::one();
    for i in 2..=n {
        result = result * BigInt::from(i);
    }
    result
}
fn main() {
    let t_init1 = SystemTime::now();
    let _result1 = factorial(10000);
    println!("Rust, N = 10000, time:{}", t_init1.elapsed().unwrap().as_nanos());

    let t_init2 = SystemTime::now();
    let _result2 = factorial(20000);
    println!("Rust, N = 20000, time:{}", t_init2.elapsed().unwrap().as_nanos());

    let t_init3 = SystemTime::now();
    let _result3 = factorial(30000);
    println!("Rust, N = 30000, time:{}", t_init3.elapsed().unwrap().as_nanos());
}