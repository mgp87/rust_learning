mod my_func;
mod other_funcs;

use crate::my_func::add_five;
use crate::other_funcs::minus_funcs::substract_ten;

// Global Constant
const MY_INT: u8 = 10;
const MSG: &str = "Hello, Rust!";

/* This function will cause a dangling reference, lifetime should be specified
* s will be dropped when the function execution finishes
* so returned value will point to an invalid memory location
* This is an example of a dangling reference
* fn make_string_dangle() -> &String {
*    let s: String = String::from("Dangle");
*    let r: &String = &s; // This will cause a dangling reference
*    r // Returning a reference to a value that will be dropped
* }
*/

fn make_string_not_dangle() -> String {
    let s: String = String::from("Dangle");
    let r: String = s; // This will not cause a dangling reference
    r // Returning a value will work
}

fn main() {
    let mut x: u32 = 50;
    x = add_five(x);
    println!("{}", x);

    x = substract_ten(x);
    println!("{}", x);

    // Memory

    // Stack
    let x: u8 = 50;
    println!("x is {}", x);

    // Heap
    let mut arr: Vec<u8> = vec![1, 2, 3, 4, 5];
    arr.push(10);
    println!("arr is {:?}", arr);

    // A Reference on the Stack pointing to a value on the Heap
    let arr_2 = &arr[0..3];
    println!("arr_2 is {:?}", arr_2);

    // Heap
    let mut s: String = String::from("Hello");
    s.push_str(" World");
    s.push('!');
    println!("s is {:?}", s);

    // A Reference on the Stack pointing to a value on the Heap
    let s_2: &str = &s[0..5];
    println!("s_2 is {:?}", s_2);

    println!("MY_INT is {}", MY_INT);

    // String Literal stored in a read-only static memory
    let msg: &str = "Hello, Rust!";
    println!("msg is {:?}", msg);

    // Dynamic String stored in the Heap
    let msg_string: String = "Hello3".to_string();
    println!("msg_string is {:?}", msg_string);

    println!("MSG is a constant with value {:?}", MSG);

    // Ownership and Borrowing
    let x: i32 = 50;
    let y: i32 = x; // y has a copy of the value
    println!("x is {}, y is {}", x, y);

    // Will not work
    let s: String = String::from("Hello");
    let t: String = s; // s is moved to t, s is no longer valid
    // println!("s is {:?}", s); It has no value since it was moved to t
    println!("t is {:?}", t); // Will work, has a value moved from s

    // Will work
    let u: String = String::from("Hello2");
    let v: String = u.clone(); // u is cloned into v with a different address
    println!("u is {:?}, v is {:?}", u, v);

    let u: String = String::from("Hello3");
    let v: &String = &u; // value of u is borrowed (not moved) into v with the same address
    println!("u is {:?}, v is {:?}", u, v);

    let s: String = make_string_not_dangle();
    println!("s is {:?}", s);
}
