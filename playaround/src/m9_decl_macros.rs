/**
 * MACRO CAPTURES
 * --------------
 *
 * expr --> expressions
 * matches to a valid rust expression
 * "hello".to_string(), vec![1, 2, 3], 1 + 2, 1
 *
 * stmt --> statements
 * matches to a rust statement
 * let x = 5, x.push(1), return Some(x)
 *
 * ident --> identifiers
 * matches to a rust identifier
 * variable name, function name, module name
 *
 * ty --> types
 * matches to a rust type
 * i32, Vec<String>, Option<T>
 *
 * path --> paths
 * matches to a rust path
 * std::collections::HashMap
 *
 * REPETITION SPECIFIER
 * --------------------
 *
 * * --> match zero or more repetitions
 * + --> match one or more repetitions
 * ? --> match zero or one repetition
 *
 * NOTE
 * ----
 *
 * To enable a macro to be used in other files if the crate
 * containing your macro is imported, add this above your macro:
 *
 * #[macro_export]
 * macro_rules! my_macro {...}
 */

#[cfg(test)]
mod tests {

    // Custom declarative macros
    macro_rules! mad_skills {
        ($x: expr) => {
            format!("Mad skills expression: {}", $x)
        };
    }

    macro_rules! mad_skills_two {
        ($x: ty) => {
            match stringify!($x) {
                "i32" => "Mad skills for i32 type".to_string(),
                "Vec<i32>" => "Mad skills for Vec<i32> type".to_string(),
                _ => "Unknown type".to_string(),
            }
        };
    }

    // Custom declarative macro using repetitions
    macro_rules! my_vec {
        // Using + means the macro can accept one or more expressions, not zero
        ($($x: expr), +) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                )+
                temp_vec
            }
        };
    }

    #[test]
    fn test_m9_decl_macros() {
        // commands ending with ! are declarative macros
        println!("Hello, Macro!");
        dbg!("Hello, Macro!");
        let x: Vec<i32> = vec![1, 2, 3];
        let formatted: String = format!("Formatted: {:?}", x);
        dbg!(formatted);

        let some_var = mad_skills!(1 + 2);
        dbg!(some_var);

        let some_type = mad_skills_two!(i32);
        dbg!(some_type);

        let y = my_vec!(1);
        dbg!(y);

        let z = my_vec!(1, 2, 3);
        dbg!(z);
    }
}
