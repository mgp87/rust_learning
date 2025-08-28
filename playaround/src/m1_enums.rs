#[cfg(test)]
mod tests {

    #[derive(Debug)]
    #[allow(dead_code, unused_variables)]
    enum CarColour {
        Red,
        Green,
        Blue,
        Silver,
    }

    #[derive(Debug)]
    enum GivenResult<T, E> {
        // Generics and Enums
        Ok(T),
        Err(E),
    }

    #[derive(Debug)]
    enum GivenOption<T> {
        // Generics and Enums
        None,
        Some(T),
    }

    fn create_car_colour_red() -> CarColour {
        let my_car_colour: CarColour = CarColour::Red;
        my_car_colour
    }

    fn check_under_five(num_check: u8) -> GivenResult<bool, String> {
        if num_check < 5 {
            println!("The number {} is under five.", num_check);
            GivenResult::Ok(true)
        } else {
            println!("The number {} is not under five.", num_check);
            GivenResult::Err("Number is not under five".into())
        }
    }

    fn check_under_five_built_in(num_check: u8) -> Result<bool, String> {
        if num_check < 5 {
            println!("The number {} is under five.", num_check);
            Ok(true)
        } else {
            println!("The number {} is not under five.", num_check);
            Err("Number is not under five".into())
        }
    }

    fn remainder_zero(num_check: f32) -> GivenOption<f32> {
        let remainder: f32 = num_check % 10.0;
        if remainder != 0.0 {
            GivenOption::Some(remainder)
        } else {
            GivenOption::None
        }
    }

    fn remainder_zero_built_in(num_check: f32) -> Option<f32> {
        let remainder: f32 = num_check % 10.0;
        if remainder != 0.0 {
            Some(remainder)
        } else {
            None
        }
    }

    #[test]
    fn test_enums() {
        let variant = create_car_colour_red();
        match variant {
            CarColour::Red => assert_eq!(1, 1),
            CarColour::Green => assert_eq!(2, 2),
            CarColour::Blue => assert_eq!(3, 3),
            CarColour::Silver => assert_eq!(4, 4),
        }

        let is_ok_result = check_under_five(3);
        match is_ok_result {
            GivenResult::Ok(value) => assert_eq!(value, true),
            GivenResult::Err(err) => panic!("Expected Ok, got Err: {}", err),
        }

        let is_not_ok_result = check_under_five(6);
        match is_not_ok_result {
            GivenResult::Ok(value) => assert_eq!(value, true),
            GivenResult::Err(err) => println!("Expected Ok, got Err: {}", err),
        }

        let is_ok_result = check_under_five_built_in(3);
        match is_ok_result {
            Ok(value) => assert_eq!(value, true),
            Err(err) => panic!("Expected Ok, got Err: {}", err),
        }

        let is_not_ok_result = check_under_five_built_in(6);
        match is_not_ok_result {
            Ok(value) => assert_eq!(value, true),
            Err(err) => println!("Expected Ok, got Err: {}", err),
        }

        let remainder = remainder_zero(25.0);
        dbg!(remainder);

        let remainder_none = remainder_zero(20.0);
        dbg!(remainder_none);

        let remainder_built_in = remainder_zero_built_in(25.0);
        dbg!(remainder_built_in);

        let remainder_none_built_in = remainder_zero_built_in(20.0);
        dbg!(remainder_none_built_in);
    }
}
