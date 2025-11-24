use super::container::Container;

pub struct Basket<T> {
    item: Option<T>,
}

impl<T> Basket<T>{
    pub fn new(item: T) -> Self {
        Basket {
            item: Some(item)
        }
    }
}

impl<T> Container<T> for Basket<T> {
    // By implementing a trait, pub is not needed anymore
    
    fn get(&mut self) -> Option<T> {
        self.item.take() // take returns a value leaving a None in place
    }

    fn put(&mut self, item: T) {
        self.item = Some(item);
    }

    fn is_empty(&self) -> bool {
        self.item.is_none()
    }
}