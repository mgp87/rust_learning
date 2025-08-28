#[cfg(test)]
mod test {
    use my_proc_macro::function_to_string;

    #[allow(dead_code)]
    const OUTPUT: &str = "";

    #[function_to_string]
    fn some_function_for_ai_llm(_param: &str) {
        /// This is a function that will be given to AI
        /// to guess and output in a structured manner
        println!("{}", OUTPUT);
    }

    #[test]
    fn test_proc_macro() {
        let x = some_function_for_ai_llm("some_input");
        dbg!(x);
    }
}
