mod m10_proc_macros;
mod m11_smart_pointers;
mod m12_concurrency;
mod m1_enums;
mod m2_structs;
mod m3_traits;
mod m4_polymorphism;
mod m5_lifetimes;
mod m6_patterns;
mod m7_async;
mod m8_collections;
mod m9_decl_macros;

fn main() {
    let y: i32 = 4;

    // for loop
    for i in 0..=y {
        if i != 4 {
            print!("{}, ", i);
        } else {
            println!("{}", i);
        }
    }

    // mutation
    let mut z: i32 = 5;
    print!("z was {}", z);
    z = 10;
    println!(", now z is {}", z);

    let freezing_temp: f64 = -2.4;
    println!("Freezing temperature is {} degrees Celsius.", freezing_temp);

    let is_zero_reminder: bool = 10 % 4 != 0;
    println!("Is 10 % 4 != 0? {}", is_zero_reminder);

    let my_char: char = 'z'; // String in "", char in ''
    println!("My character is '{}'.", my_char);

    let emoji: char = '😊';
    println!("My emoji is '{}'.", emoji);

    let my_floats: [f32; 10] = [0.0; 10];
    println!("My array of floats: {:?}", my_floats);

    let my_floats_new: [f32; 10] = my_floats.map(|n| n + 2.0);
    println!("My new array of floats: {:?}", my_floats_new);

    let name: &str = "Miguel";
    println!("My name is '{}'.", name);

    let dynamic_name: String = String::from("Miguel GP");
    println!("My dynamic name is '{}'.", dynamic_name);
    println!("My dynamic name stored in memory is '{:p}'.", &dynamic_name); // :p to print address

    let dynamic_name: String = name.to_string();
    println!("My dynamic name from static string is '{}'.", dynamic_name);

    let str_slice: &str = &dynamic_name[0..5];
    println!("My string slice is '{}'.", str_slice);

    let mut chars: Vec<char> = Vec::new();
    chars.insert(0, 'h');
    chars.insert(1, 'e');
    chars.insert(2, 'l');
    chars.push('l');
    chars.push('o');
    chars.push('!');
    println!("My characters are: {:?}", chars);
    dbg!(&chars);

    let removed_char: char = chars.pop().unwrap();
    println!("Removed character: {}", removed_char);

    chars.iter().for_each(|c| println!("char is {}", c));

    let chars_again: Vec<char> = vec!['h', 'e', 'l', 'l', 'o'];
    println!("My characters again are: {:?}", chars_again);
    dbg!(&chars_again);

    let collected: String = chars_again.iter().collect();
    println!("My collected string is '{}'.", collected);
    dbg!(collected);

    for c in chars_again {
        print!("{}", c);
        if c == 'o' {
            println!("world!");
        }
    }

    let num: i32 = 5;
    let add_num = |x: i32| x + num;
    let new_num: i32 = add_num(10);
    println!("New number after adding 10 is: {}", new_num);

    println!("Big number is {}", 98222888);
    println!("Big number is {}", 98_222_888);
    println!("Hex is {}", 0xff);
    println!("Octal is {}", 0o77);
    println!("Binary is {}", 0b1111_0000);
    println!("Bytes 'A' is {}", b'A');

    // Raw String Literal
    let text: &str =
        r#"{"message": "This is a raw string literal with "quotes" and \backslashes\."}"#;
    dbg!(text);

    let a: u8 = 0b_1010_1010;
    let b: u8 = 0b_0101_1010;
    println!("a's value is {}", a);
    println!("b's value is {}", b);
    println!("a's value is {:08b}", a);
    println!("b's value is {:08b}", b);

    // Logic gates
    println!("AND gate: {}", a & b);
    println!("NAND gate: {}", !(a & b));
    println!("OR gate: {}", a | b);
    println!("NOR gate: {}", !(a | b));
    println!("XOR gate: {}", a ^ b);
    println!("XNOR gate: {}", !(a ^ b));
    println!("NOT gate: {}", !a);

    println!("AND gate: {:08b}", a & b);
    println!("NAND gate: {:08b}", !(a & b));
    println!("OR gate: {:08b}", a | b);
    println!("NOR gate: {:08b}", !(a | b));
    println!("XOR gate: {:08b}", a ^ b);
    println!("XNOR gate: {:08b}", !(a ^ b));
    println!("NOT gate: {:08b}", !a);

    // Bitwise Operations
    println!("a's value is {:08b}", a);
    println!("a << 1: {:08b}", a << 1);
    println!("a's value is {:08b}", a);
    println!("a >> 1: {:08b}", a >> 1);

    println!("a's value is {}", a);
    println!("a << 1: {}", a << 1);
    println!("a's value is {}", a);
    println!("a >> 1: {}", a >> 1);

    // Little Endian or Big Endian
    let n: u16 = 0x1234;
    println!("n is {:?}", n);
    println!("n is {:016b}", n);

    let big_endian: [u8; _] = n.to_be_bytes();
    let little_endian: [u8; _] = n.to_le_bytes();
    println!("Big Endian: {:?}", big_endian);
    println!("Little Endian: {:?}", little_endian);
    println!("Big Endian: {:02X}{:02X}", big_endian[0], big_endian[1]);
    println!(
        "Little Endian: {:02X}{:02X}",
        little_endian[0], little_endian[1]
    );
}
