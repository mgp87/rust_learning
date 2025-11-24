// use std::sync::{Arc, mpsc};
// use std::thread;
// use std::sync::{Arc, Mutex};
use rayon::prelude::*;
use num::{BigUint, One};
use std::time::Instant;

fn factorial(num: u32) -> BigUint {
    if num == 0 || num == 1 {
        return BigUint::one()
    }else{
        (1..=num).map(BigUint::from).reduce(|acc, x| acc * x).unwrap()
    }
}

// Usin Rayon crate for paralellism
fn multi_fact(num: u32) -> BigUint {
    if num == 0 || num == 1 {
        BigUint::one()
    }else{
        (1..=num).into_par_iter().map(BigUint::from).reduce(|| BigUint::one(), |acc, x| acc * x)
    }
}

fn main() {
    let now = Instant::now();
    factorial(50000);
    println!("{:.2?}", now.elapsed());

    let now = Instant::now();
    multi_fact(50000); // paralell computation
    println!("{:.2?}", now.elapsed());

    //println!("{}", factorial(3));

    //println!("{}", multi_fact(3));
}