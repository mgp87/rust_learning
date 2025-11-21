// If each "thing" have same methods --> USE ENUM
// If each "thing" have some same, but some different methods --> USE STRUCTS
// If each "thing" have many "fields", you might want to use STRUCTS too
// If each "thing" is different, then definitely --> USE STRUCTS

#[derive(Debug)]
enum Media {
    Book {title: String, author: String},
    Movie {title: String, director: String},
    Audiobook {title: String},
    Podcast (u32), // Podcast {episode_number: u32}
    Placeholder,
}

impl Media {
    fn description_verbose(&self) -> String {
        // Basic verbose type check
        if let Media::Book {title, author} = self {
            format!("Book: {} {}", title, author)
        }else if let Media::Movie {title, director} = self {
            format!("Movie: {} {}", title, director)
        }else if let Media::Audiobook{title} = self {
            format!("Audiobook: {}", title)
        }else{
            String::from("No media found")
        }
    }

    fn description_pattern_matching(&self) -> String {
        match self {
            Media::Book {title, author} => format!("Book: {} {}", title, author),
            Media::Movie {title, director} => format!("Movie: {} {}", title, director),
            Media::Audiobook {title} => format!("Audiobook: {}", title),
            Media::Podcast (episode_number) => format!("Podcast: {}", episode_number),
            Media::Placeholder => format!("Placeholder")
        }
    }
}

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>
}

impl Catalog {
    fn new() -> Self {
        Catalog {
            items: vec![]
        }
    }

    fn add(&mut self, media: Media) {
        self.items.push(media);
    }

    fn get_by_index_custom(&self, index: usize) -> MightHaveAValue {
        if self.items.len() > index {
            // There's a value
            MightHaveAValue::ThereIsAValue(&self.items[index])
        }else {
            // No value to return
            MightHaveAValue::NoValueAvailable
        }
    }

    fn get_by_index(&self, index: usize) -> Option<&Media> {
        if self.items.len() > index {
            // There's a value
            Some(&self.items[index])
        }else {
            // No value to return
            None
        }
    }
}

enum MightHaveAValue<'a> { // Simulating Option enum
    ThereIsAValue(&'a Media),
    NoValueAvailable, 
}

fn main() {
    let audiobook = Media::Audiobook {
        title: String::from("Audiobook"),
    };
    let good_movie = Media::Movie {
        title: String::from("Good Movie"),
        director: String::from("Good Director"),
    };
    let bad_book = Media::Book {
        title: String::from("Bad Book"),
        author: String::from("Bad Author"),
    };
    let podcast = Media::Podcast(10);
    let placeholder = Media::Placeholder;

    let mut catalog = Catalog::new();
    catalog.add(audiobook);
    catalog.add(good_movie);
    catalog.add(bad_book);
    catalog.add(podcast);
    catalog.add(placeholder);
    println!("{:#?}", catalog);

    println!("{:#?}", catalog.items.get(0)); // Option type --> Some(value)
    println!("{:#?}", catalog.items.get(100)); // Option type --> None

    match catalog.items.get(0) {
        Some(value) => {
            println!("Item: {:#?}", value);
        }
        None => {
            println!("Nothing at this index");
        }
    }

    // ------------------------- Custom enum ---------------------------------------

    let item = catalog.get_by_index_custom(4);

    match catalog.get_by_index_custom(0) {
        MightHaveAValue::ThereIsAValue(value) => {
            println!("{:#?}", value);
        }
        MightHaveAValue::NoValueAvailable => {
            println!("No value here");
        }
    }

    if let MightHaveAValue::ThereIsAValue(value) = catalog.get_by_index_custom(0) {
        println!("Pattern matching --> {:#?}", value);
    }else{
        println!("No value here");
    }

    // ------------------------- Option enum ---------------------------------------

    match catalog.get_by_index(0) {
        Some(value) => {
            println!("{:#?}", value);
        }
        None => {
            println!("No value here");
        }
    }

    if let Some(value) = catalog.get_by_index(0) {
        println!("Pattern matching --> {:#?}", value);
    }else{
        println!("No value here");
    }

    // ------------------------- unwrap ---------------------------------------
    
    let item_none = catalog.get_by_index(40); // None -> panic with unwrap
    let item_none_unwrap = item_none.unwrap(); // Panic!
    let item_none_expect = item_none.expect("Value here expected");
    let placeholder = Media::Placeholder;
    let item_none_unwrap_or = item_none.unwrap_or(&placeholder); // if none, it returns placeholder as fallback value
    let item_some = catalog.get_by_index(4);
    let item_some_unwrap = item_some.unwrap(); // Some -> gets value
    println!("{:#?}", item_some_unwrap);
}
