fn main() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    println!("r1 is {:?}", r1);
    println!("r2 is {:?}", r2);



    // println!("r1 is {:?}", *r1); // this gives error because raw pointers may be null, dangling or unaligned so they can cause data races.
    // println!("r2 is {:?}", *r2);

    // to avoid this, we can use unsafe code block:

    unsafe {
        println!("r1 is {:?}", *r1); // this gives error because raw pointers may be null, dangling or unaligned so they can cause data races.
        println!("r2 is {:?}", *r2);
    }

    // when to use unsafe code:
    // using c interfaces
    // low level programming to access memory or custom memory allocator
    // performance critical algorithm
    // custom data structures with control over mermory allocation

    // it can come with risks like undefined behaviour
}
