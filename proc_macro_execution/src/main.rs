use proc_macros::debug_print;

// Declarative Macros are used to match a given pattern and replace it with
// specified code. They are defined using the macro_rules! macro and then
// they can be used to define new control structures or simplify repetitive code

macro_rules! average { // evaluated at compile time
    ($(,)*) => {{ // $(,)* is a pattern in this case it matches 0 or more comma separated items
        0.0
    }};

    ($($val:expr), + $(,)*) => {{ // this $($val:expr), + $(,)* represents a non-empty list separated by commas
        let count = 0usize $(+ { let _ = stringify!($val); 1})*; // initialize the variable to 0. we count the number of values
        let sum = 0.0 $(+ $val as f64)*;
        sum / count as f64
    }};
}

#[debug_print]
fn main() {
    println!("Hello, world!");
    println!("{}", average!()); // use of the macro
    println!("{}", average!(1.0, 2.0, 3.0));
    println!("{}", average!(1,2,3,4,5));
}
