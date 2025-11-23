#[derive(Debug)]
pub enum Media {
    Book {title: String, author: String},
    Movie {title: String, director: String},
    Audiobook {title: String},
    Podcast (u32), // Podcast {episode_number: u32}
    Placeholder,
}

impl Media {
    pub fn description_verbose(&self) -> String {
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

    pub fn description_pattern_matching(&self) -> String {
        match self {
            Media::Book {title, author} => format!("Book: {} {}", title, author),
            Media::Movie {title, director} => format!("Movie: {} {}", title, director),
            Media::Audiobook {title} => format!("Audiobook: {}", title),
            Media::Podcast (episode_number) => format!("Podcast: {}", episode_number),
            Media::Placeholder => format!("Placeholder")
        }
    }
}