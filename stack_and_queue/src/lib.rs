/// STACK:
/// ------
/// - specific order in how we insert and remove elements
/// - LIFO: Last In First Out.
/// - Insert at the top -> Pick from the top like a Stack of plates
/// 
/// 
/// QUEUE:
/// ------
/// - specific order
/// - FIFO: First In First Out
/// - Insert at the top -> Pick from the bottom
/// 

use std::collections::LinkedList;

pub struct Queue<T> {
    element: LinkedList<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            element: LinkedList::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.element.push_back(value);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.element.pop_front()
    }

    pub fn peek(&self) -> Option<&T> {
        self.element.front()
    }

    pub fn length(&self) -> usize {
        self.element.len()
    }

    pub fn is_empty(&self) -> bool {
        self.element.is_empty()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_queue() {
        let mut q = Queue::new();

        q.enqueue(1);
        q.enqueue(2);
        q.enqueue(3);
        q.enqueue(4);
        q.enqueue(5);

        assert_eq!(q.length(), 5);
        assert_eq!(q.is_empty(), false);

        let e = q.dequeue();
        assert_eq!(e, Some(1));

        let f = q.peek();
        assert_eq!(f, Some(&2));
    }
}