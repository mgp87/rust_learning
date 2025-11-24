pub trait Container<T> {
    fn get(&mut self) -> Option<T>; // abstract method
    fn put(&mut self, item: T);
    fn is_empty(&self) -> bool;
}