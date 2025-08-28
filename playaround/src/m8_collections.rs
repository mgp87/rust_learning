#[cfg(test)]
mod test {
    use std::collections::{HashMap, HashSet};

    #[test]
    fn test_hashmap() {
        // HashMaps store key-value pairs
        // For instance:
        // &str -> Person
        // u8 -> &str
        // &str -> u32
        let person1 = "Alice";
        let person2 = "Bob";
        let mut results_hm: HashMap<&str, u32> = HashMap::new();
        results_hm.insert(person1, 55);
        results_hm.insert(person2, 51);

        let test_score: Option<&u32> = results_hm.get(person1);
        dbg!(test_score.unwrap());

        if results_hm.contains_key("Alice") {
            dbg!("Alice is here bruh");
        }
    }

    #[test]
    fn test_hashset() {
        let mut names_hs: HashSet<&str> = HashSet::new();
        names_hs.insert("Alice");
        names_hs.insert("Bob");
        names_hs.insert("Charlie");

        if names_hs.contains("Bob") {
            dbg!("Bob is here bruh");
        }
    }
}
