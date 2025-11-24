use std::fs;
use std::io::Error;

// fn divide(a: f64, b:f64) -> Result<f64, std::io::Error> {
//    if b == 0.0 {
        // division by 0
//        Err(Error::other("b is 0"))
//    }else{
//        Ok(a / b)
//    }
// }

// fn validate_email(email: String) -> Result<(), Error>{
//    if email.contains("@") {
//        Ok(()) // () means an empty tuple which is the return in the "ok position", when the return is nothing, the convention is to return an empty tuple
//    } else {
//        Err(Error::other("Emails must have @ symbol"))
//    }
// }

// fn string_test(
//    a: String, // String::from("red") --> When we need ownership of text or text that can grow or shrink; stack and heap
//    b: &String, // &String::from("red") --> Read only ref --> rarely used, rust turns &String into &str automatically
//    c: &str // String slice --> "red" / String::from("red").as_str() --> read only ref --> When we do not need ownership or we want to refer to a portion of a string owned by something else; Stack that refers to heap-allocated or data-allocated text
// ) {
//    return;
// }

fn extract_errors(text: &str) -> Vec<&str>{
    let split_text = text.split("\n");
    let mut results = vec![];
    for line in split_text {
        if line.starts_with("ERROR"){
            results.push(line);
        }
    }
    results
}

fn main() -> Result<(), Error> {
    //string_test(
    //    String::from("red"), // "red".to_string() is teh same
    //    &String::from("red"),
    //    "red" // String::from("red").as_str() is the same
    // );

    // match divide(5.0, 3.0) {
    //    Ok(result_of_division) => {
    //        println!("{:#?}", result_of_division);
    //    }
    //    Err(what_went_wrong) => {
    //        println!("{:#?}", what_went_wrong);
    //    }
    //}

    // match divide(5.0, 0.0) {
    //    Ok(result_of_division) => {
    //        println!("{:#?}", result_of_division);
    //    }
    //    Err(what_went_wrong) => {
    //        println!("{:#?}", what_went_wrong);
    //    }
    //}

    // match validate_email(String::from("asfds@asdfs.com")) {
    //    Ok(..) => println!("Validated email"), // .. means we disregard the potential value
    //    Err(reason) => println!("{}", reason),
    // }

    // match validate_email(String::from("asfds_asdfs.com")) {
    //    Ok(..) => println!("Validated email"), // .. means we disregard the potential value
    //    Err(reason) => println!("{}", reason),
    // }

    // match fs::read_to_string("zfdsxgcvlogs.txt") {
    //    Ok(text) => println!("{}", text.len()),
    //    Err(read_err) => println!("{}", read_err),
    // }

    // let mut error_logs = vec![];
    // match fs::read_to_string("logs.txt") {
        // Ok(text) => {
        //    let error_logs = extract_errors(text.as_str());
        //    match fs::write("errors_match.txt", error_logs.join("\n")) {
        //        Ok(..) => println!("Wrote errors_match.txt"),
        //        Err(reason_of_failure) => {
        //            println!("Writing of errors_match.txt failed: {}", reason_of_failure);
        //        }
        //    }
        // } 
        // text variable is dropped here after exiting scope so error_logs &str pointers will have no value to point to
        // By returning Vec<String> instead of Vec<&str>, we solve the error since we are generating copies that are owned
        // Err(read_err) => println!("{}", read_err),
    // }

    // Equivalent to the nested match statements but might panic:
    // let text = fs::read_to_string("logs.txt").expect("failed to read logs.txt");
    // let error_logs = extract_errors(text.as_str());
    // fs::write("errors.txt", error_logs.join("\n")).expect("failed to write errors.txt");

    // Returning Result<(), Error> --> ? operator to unwrap and return values from Result enum: Ok(..) or Error
    let text = fs::read_to_string("logs.txt")?;
    let error_logs = extract_errors(text.as_str());
    fs::write("errors_?.txt", error_logs.join("\n"))?;
    
    Ok(())

    // - Use match or "if let" when we want to meaningfully deal with an error besides logging
    // - Call unwrap() or expect() for quick debugging or if we want to crach on an Err()
    // - Use try operator ("?") to unwrap or propagate the Result when we do not have any way to handle the error in the current function
}
