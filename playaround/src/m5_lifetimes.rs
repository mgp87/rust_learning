#[allow(dead_code, unused_variables)]
fn example_0() {
    let r: &i32;
    //{
    let x: i32 = 5;
    r = &x;
    //}
    println!("r: {}", r); // This will cause a compile-time error because x is dropped
}

#[allow(dead_code, unused_variables)]
fn example_1() {
    // Allocate space in memory
    let highest_age: i32;

    // Initialize vars
    let alice_age: i32 = 20; // lifetime 'a
    let bob_age: i32 = 21; // lifetime 'b : 'a

    // Call function
    highest_age = largest(&alice_age, &bob_age);

    // Print output
    println!("The highest age is: {}", highest_age);

    fn largest(compare_1: &i32, compare_2: &i32) -> i32 {
        if compare_1 > compare_2 {
            *compare_1
        } else {
            *compare_2
        }
    }
}

#[allow(dead_code, unused_variables)]
fn example_2() {
    // Allocate space in memory
    let highest_age: &i32;

    // Initialize vars
    let alice_age: i32 = 20; // lifetime 'a
    let bob_age: i32 = 21; // lifetime 'b : 'a

    // Call function
    highest_age = largest(&alice_age, &bob_age);

    // Print output
    println!("The highest age is: {}", highest_age);

    // lifetime generics
    fn largest<'a>(compare_1: &'a i32, compare_2: &'a i32) -> &'a i32 {
        if compare_1 > compare_2 {
            compare_1
        } else {
            compare_2
        }
    }
}

#[allow(dead_code, unused_variables)]
fn example_3() {
    // Allocate space in memory
    let highest_age: &i32;
    let new_value: i32;

    // Initialize vars
    let alice_age: i32 = 20; // lifetime 'a
    {
        let bob_age: i32 = 21; // lifetime 'b

        // Call function
        highest_age = largest(&alice_age, &bob_age);
        new_value = *highest_age;
    } // 'b is out of scope from here

    // Print output
    println!("The highest age is: {}", new_value);

    // lifetime generics
    fn largest<'a>(compare_1: &'a i32, compare_2: &'a i32) -> &'a i32 {
        if compare_1 > compare_2 {
            compare_1
        } else {
            compare_2
        }
    }
}

#[allow(dead_code, unused_variables)]
fn example_4() {
    // Allocate space in memory
    let highest_age: &i32;
    let new_value: i32;

    // Initialize vars
    let alice_age: i32 = 20; // lifetime 'a
    {
        let bob_age: i32 = 21; // lifetime 'b

        // Call function
        highest_age = largest::<i32>(&alice_age, &bob_age);
        new_value = *highest_age;
    } // 'b is out of scope from here

    // Print output
    println!("The highest age is: {}", new_value);

    // lifetime generics
    fn largest<'a, T: PartialOrd>(compare_1: &'a T, compare_2: &'a T) -> &'a T {
        if compare_1 > compare_2 {
            compare_1
        } else {
            compare_2
        }
    }
}

#[allow(dead_code, unused_variables)]
struct Person<'p> {
    name: &'p str,
    points: &'p f32,
}

#[allow(dead_code, unused_variables)]
fn example_5() {
    // Allocate space in memory
    let highest_age: &f32;
    let new_value: f32;

    // Initialize vars
    let alice: Person = Person {
        name: "Alice",
        points: &10.2,
    };
    {
        let bob: Person = Person {
            name: "Bob",
            points: &12.5,
        };

        // Call function
        highest_age = largest::<f32>(alice.points, bob.points);
        new_value = *highest_age;
    } // 'b is out of scope from here

    // Print output
    println!("The highest age is: {}", new_value);

    // lifetime generics
    fn largest<'a, T: PartialOrd>(compare_1: &'a T, compare_2: &'a T) -> &'a T {
        if compare_1 > compare_2 {
            compare_1
        } else {
            compare_2
        }
    }
}
