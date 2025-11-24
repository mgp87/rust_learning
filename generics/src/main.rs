use num_traits::{ToPrimitive, Float};

//fn solve(a: f64, b: f64) -> f64 {
//    (a.powi(2) + b.powi(2)).sqrt()
//}

fn solve<T: Float>(a: T, b: T) -> f64 { // <T: Trait> --> you can pass any type that implements trait Trait, in this case Float trait
    let a_f64 = a.to_f64().unwrap();
    let b_f64 = b.to_f64().unwrap();

    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}

fn solve_plus<T: Float, U: Float>(a: T, b: U) -> f64 {
    let a_f64 = a.to_f64().unwrap();
    let b_f64 = b.to_f64().unwrap();

    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}

fn solve_plus_plus<T: ToPrimitive, U: ToPrimitive>(a: T, b: U) -> f64 {
    let a_f64 = a.to_f64().unwrap();
    let b_f64 = b.to_f64().unwrap();

    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}

fn main() {
    let a: f32 = 3.0;
    let b: f32 = 4.0; // defaults to f64

    // let a_f64 = a as f64; // casting to f64
    // let a_f64_second = a.to_f64().unwrap(); // casting to f64

    println!("{}", solve::<f32>(a, b));

    let c: f32 = 3.0;
    let d: f32 = 4.0;

    println!("{}", solve_plus(c, d));

    let e: i32 = 3;
    let f: f32 = 4.0;

    println!("{}", solve_plus_plus(e, f));
}
