use crate::Media;

#[derive(Debug)]
pub struct Catalog {
    pub items: Vec<Media>
}

impl Catalog {
    pub fn new() -> Self {
        Catalog {
            items: vec![]
        }
    }

    pub fn add(&mut self, media: Media) {
        self.items.push(media);
    }

    pub fn get_by_index_custom(&self, index: usize) -> MightHaveAValue {
        if self.items.len() > index {
            // There's a value
            MightHaveAValue::ThereIsAValue(&self.items[index])
        }else {
            // No value to return
            MightHaveAValue::NoValueAvailable
        }
    }

    pub fn get_by_index(&self, index: usize) -> Option<&Media> {
        if self.items.len() > index {
            // There's a value
            Some(&self.items[index])
        }else {
            // No value to return
            None
        }
    }
}

pub enum MightHaveAValue<'a> { // Simulating Option enum
    ThereIsAValue(&'a Media),
    NoValueAvailable, 
}