fn print_elements_for(elements: &[String]) { // Vector Slice: &[type] --> it works with full vectors or slices (subsets of vectors) / &Vec<type> works with full vectors only
    println!("for loop");
    for element in elements {
        println!("{}", element);
    }
}

fn print_elements_for_each(elements: &[String]) {
    println!("for each loop");
    elements.iter().for_each(|elem| println!("{}", elem));
}

fn print_elements_map(elements: &[String]) {
    println!("for each loop");
    elements
        .iter()
        .map(|elem| format!("{} {}", elem, elem)) // iterator adaptor, you need to use map with an iterator consumer
        .for_each(|elem| println!("{}", elem));
}

fn shorten_strings(elements: &mut [String]) {
    println!("Shorten Strings");
    // iter() gives a read_only reference
    // iter_mut() gives a mutable reference
    // into_iter() gives ownership of each element unless called on a mutable ref of a vector
    elements.iter_mut().for_each(|elem| elem.truncate(1));
}

fn to_uppercase(elements: &[String]) -> Vec<String> {
    elements
        .iter()
        .map(|elem| elem.to_uppercase())
        .collect() // iterator consumer
        // to specify types in a collect call (called "Turbofish"):
        // .collect::<Vec<String>>() or for infering the type .collect::<Vec<_>>
        // .collect::<HashMap<String>>() or for infering the type .collect::<HashMap<_>>
        // .collect::<LinkedList<String>>() or for infering the type .collect::<LinkedList<_>>
}

fn move_elements(vec_a: Vec<String>, vec_b: &mut Vec<String>) {
    vec_a.into_iter().for_each(|elem| vec_b.push(elem));
}

fn explode(elements: &[String]) -> Vec<Vec<String>> { // Vec<String> --> Vec<Vec<String>>
    elements
        .iter()
        .map(|elem| elem.chars().map(|c| c.to_string()).collect()) // chars returns an iterator over each char
        .collect()
}

fn find_color_or(elements: &[String], search: &str, fallback: &str) -> String {
    elements
        .iter()
        .find(|elem| elem.contains(search)) 
        .map_or( // 
            String::from(fallback),
            |elem| elem.to_string()
        )
    // find(||): calls next on the iterator until finds a truthy value, returns Some(value) if found or None if not
    // map_or(if None arg, if Some arg): if an Option method that takes two arguments, first if None, second if Some(value)
}

fn main() {
    let mut colors = vec![String::from("red"), String::from("green"),String::from("blue"),];
    println!("Og vector: {:#?}", colors);

    print_elements_for(&colors);
    print_elements_for_each(&colors);
    print_elements_map(&colors);

    shorten_strings(&mut colors);
    println!("Shortened vector: {:#?}", colors);

    let uppercase = to_uppercase(&colors);
    println!("Uppercase vec: {:#?}", uppercase);

    let mut destination = vec![];
    move_elements(colors, &mut destination);
    println!("Destination: {:#?}", destination);

    let exploded = explode(&destination);
    println!("Exploded: {:#?}", exploded);

    let colors_two = vec![String::from("red"), String::from("green"),String::from("blue"),];
    let found_color = find_color_or(&colors_two, "re", "orange");
    println!("Found or not: {:#?}", found_color);

    let found_color_two = find_color_or(&colors_two, "noooo", "orange");
    println!("Found or not: {:#?}", found_color_two);

    // filter(||): iterator adaptor that yields each value that matches the boolean statement or None otherwise:
    // a.iter().filter(|x| x.is_positive());
}
