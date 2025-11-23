mod content;

use content::{
    media::Media,
    catalog::{
        Catalog, MightHaveAValue
    }
};

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
