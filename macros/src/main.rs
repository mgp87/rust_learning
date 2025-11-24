// function like macro, more flexible syntax and more capabilities
macro_rules! gcd {
    ($a: expr, $b: expr) => { // expr for expresion
        {
            let mut m = $b;
            let mut n = $a;

            while m != 0 {
                if m < n {
                    let t = m;
                    m = n;
                    n = t;
                }
                m = m % n;
            }
            n
        }
    };
}

fn main() {
    println!("{}", gcd!(14, 15));
}
