fn next_language<'a>(languages: &'a [String], current: &str) -> &'a str { // Lifetime annotations
    let mut found = false;
    for lang in languages {
        if found {
            return lang;
        }
        if lang == current {
            found = true;
        }
    }

    languages.last().unwrap() // last returns Option<&T> so we need unwrap here
}

fn last_language(languages: &[String]) -> &str {
    languages.last().unwrap()
}

fn longest_language<'a>(lang_a: &'a str, lang_b: &'a str) -> &'a str {
    if lang_a.len() >= lang_b.len() {
        lang_a
    }else {
        lang_b
    }
}

fn main() {
    let languages = vec![
        String::from("rust"),
        String::from("go"),
        String::from("typescript"),
    ];
    let result = next_language(&languages, "go");
    println!("{}", result);

    let result_last = last_language(&languages);
    println!("{}", result_last);

    let longest = longest_language("go", "rust");
    println!("{}", longest);
}
