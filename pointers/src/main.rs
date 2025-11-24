use std::rc::Rc;
use std::cell::RefCell;

struct Flagger {
    is_true: RefCell<bool>,
}

struct FlaggerRc {
    is_true: Rc<RefCell<bool>>,
}

fn main() {
    // Smart pointers
    // Box --> allocate data on the heap instead of the stack but the pointer to the heap data will remain on the stack
    let t = (12, "eggs"); // Created on the stack
    let b = Box::new(t); // created on the heap, but b was stored on the stack
    println!("{:?}", b);

    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y); // * gives the value at the memory address

    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    println!("{:?}", y);

    // Rc --> Reference counter: allows multiple ownerships, allocated on the heap and tracks the number of references to evaluate whether a value is still in use, 0 refs = value cleared
    // Arc --> the same for threading
    let s1 = Rc::new(String::from("Pointer"));
    let s2 = s1.clone(); // creates another pointer to the value and increments the reference counter
    let s3 = s2.clone();
    println!("{}, {}, {}", s1, s2, s3);
    println!("{}, {}, {}", s1.contains("Point"), s2, s3); // Allows to use the value normally

    // RefCell<T> --> RefCell<T> is a smart pointer that allows mutable access to data even when the RefCell itself is immutable, by enforcing Rust’s borrowing rules at runtime instead of compile time.
    // RefCell provides interior mutability, meaning you can mutate data through it even if the outer structure is not mutable.
        // It replaces compile-time borrow checking with runtime borrow checking.
        // It allows:
        // Multiple immutable borrows (borrow())
        // One mutable borrow (borrow_mut())
        // Violating these rules causes a runtime panic instead of a compile-time error.
        // Often used together with Rc<T> to allow shared ownership + mutation in single-threaded code.
        // Not thread-safe (for multi-threading, use Mutex or RwLock instead).
    
    let flag = Flagger{is_true: RefCell::new(true)};
    // borrow returns Ref<T>
    // borrow_mut returns RefMut<T>
    let reference = flag.is_true.borrow();
    println!("{}", reference);

    let mut mut_ref = flag.is_true.borrow_mut();
    *mut_ref = false; // dereference first to access inside
    println!("{}", mut_ref);
    // this crashes because we created several references, we need to pair it with Rc for this

    let flag = FlaggerRc{is_true:Rc::new(RefCell::new(true))};
    // borrow returns Ref<T>
    // borrow_mut returns RefMut<T>
    let reference = Rc::new(flag.is_true.clone());
    println!("{:?}", reference);

    let mut mut_ref = reference.borrow_mut();
    *mut_ref = false;
    println!("{}", mut_ref);

}
