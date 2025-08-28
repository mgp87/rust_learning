#[cfg(test)]
mod tests {

    #[derive(Debug)]
    enum Message {
        Quit,
        ChangeColour(i32, i32, i32),
        Move { x: i32, y: i32 },
        Write(String),
    }

    fn process_message(msg: Message) {
        match msg {
            Message::Quit => println!("Quitting"),
            Message::ChangeColour(r, g, b) => {
                println!("Changing colour to red: {}, green: {}, blue: {}", r, g, b);
            }
            Message::Move { x, y: new_name } => {
                println!("Moving to x: {}, y as new_name: {}", x, new_name);
            }
            Message::Write(text) => {
                println!("Writing message: {}", text);
            }
        }
    }

    #[test]
    fn test_match_literals() {
        let number = 20;
        match number {
            1 => println!("One"),
            2 => println!("Two"),
            3 => println!("Three"),
            4 => println!("Four"),
            5 => println!("Five"),
            6 | 7 | 20 => println!("Six, Seven or Twenty"),
            _ => println!("It was something else (Default)"),
        }

        let res = match number {
            1 => "One",
            2 => "Two",
            3 => "Three",
            4 => "Four",
            5 => "Five",
            6 | 7 | 20 => "Six, Seven or Twenty",
            _ => "It was something else (Default)",
        };

        println!("Result is {}", res);
    }

    #[test]
    fn tests_match_option() {
        let some_number: Option<i32> = Some(10); // Some value of the enum Some
        println!("Some number is {:?}", some_number);
        // let prob_none: Option<i32> = None;

        let result = match some_number {
            Some(i) => i,
            None => {
                panic!("No value found");
            }
        };
        println!("Result is {}", result);

        let my_int: i32 = if let Some(i) = some_number {
            i
        } else {
            panic!("No value found");
        };

        println!("My int is {}", my_int);
    }

    #[test]
    #[allow(unused_variables)]
    fn tests_match_result() {
        let some_res: Result<i32, &str> = Ok(10);
        let some_err: Result<i32, &str> = Err("Error occurred");

        let res = match some_res {
            Ok(i) => i,
            Err(e) => panic!("Result is Err with error: {}", e),
        };

        println!("Result is {}", res);

        let my_int: i32 = if let Ok(r) = some_res {
            r
        } else {
            panic!("No value found");
        };

        println!("My int is {}", my_int);
    }

    #[test]
    fn tests_match_enum() {
        let my_quit: Message = Message::Quit;
        process_message(my_quit);

        let my_colour: Message = Message::ChangeColour(255, 10, 20);
        process_message(my_colour);

        let my_move: Message = Message::Move { x: 10, y: 20 };
        process_message(my_move);

        let my_write: Message = Message::Write(String::from("Hello, World!"));
        process_message(my_write);
    }

    #[test]
    fn tests_match_guard() {
        let pair = (10, 20);
        match pair {
            (x, y) if x == y => println!("Both are equal: {}", x),
            (x, y) if x > y => println!("First is greater: {}", x),
            (x, y) if x + y == 0 => println!("They neutralize"),
            (_, y) if y == 2 => println!("y is 2"),
            _ => println!("No match found"),
        }
    }

    #[test]
    #[allow(unused_variables)]
    fn tests_match_struct() {
        struct Location {
            x: i32,
            y: i32,
        }

        let location = Location { x: 10, y: 20 };
        match location {
            Location { x, y: 0 } => println!("Y is on the axis"),
            Location { x: 0, y } => println!("X is on the axis"),
            Location { x, y } => println!("They're not on the axis"),
        }
    }
}
