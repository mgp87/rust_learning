mod basket;
mod stack;
mod container;

use basket::Basket;
use stack::Stack;

use crate::container::Container;

fn add_string<T: Container<String>>(c: &mut T, s: String) { // any type that implements trait Container using String as type
    c.put(s);
}

fn main() {
    let mut b1 = Basket::new(String::from("item_1"));
    let b2 = Basket::new(10);
    let b3 = Basket::new(true);

    let mut s1 = Stack::new(vec![String::from("stack_1")]);
    let s2 = Stack::new(vec![1,2,3]);
    let s3 = Stack::new(vec![true, false, false, true]);

    add_string(&mut b1, String::from("hi_test"));
    add_string(&mut s1, String::from("hi_test"));
}
