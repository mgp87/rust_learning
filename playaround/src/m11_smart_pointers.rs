#[cfg(test)]
mod test {
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};

    #[test]
    fn test_box_smart_pointer() {
        #[derive(Debug)]
        #[allow(dead_code)]
        struct Node {
            id: u32,
            next_node: Option<Box<Node>>, // stores in the heap instead of the stack making it dynamic and avoiding recursion issues
        }

        // linked list kind of
        let nodes: Box<Node> = Box::new(Node {
            id: 0,
            next_node: Some(Box::new(Node {
                id: 1,
                next_node: Some(Box::new(Node {
                    id: 2,
                    next_node: None,
                })),
            })),
        });

        dbg!(nodes);
    }

    #[test]
    #[allow(dead_code)]
    fn test_reference_counter() {
        let mut x = Rc::new(50);
        let y = Rc::clone(&x);
        x = Rc::new(70);
        dbg!(x); // x now holds the new value of 70
        dbg!(y); // y still holds the original value of 50

        let z = Rc::new(RefCell::new(50));
        let w = Rc::clone(&z);
        *z.borrow_mut() = 70;
        dbg!(&z); // z now holds the new value of RefCell(70)
        dbg!(&w); // w holds the new value of RefCell(70)
        // Using RefCell allows for interior mutability, so we can change the value inside the Rc even when there are multiple references to it.
        // And the references will see the updated value.
        dbg!(z.borrow()); // 70
        dbg!(w.borrow()); // 70

        #[derive(Debug)]
        struct House {
            address_number: u16,
            street: String,
            furniture: RefCell<Vec<Rc<Furniture>>>,
        }

        #[derive(Debug)]
        struct Furniture {
            id: String,
            description: String,
            house: Weak<House>, // Weak is similar to & but doesn't count towards ownership and allows the object to be dropped
        }

        let house1 = Rc::new(House {
            address_number: 123,
            street: String::from("Main St"),
            furniture: RefCell::new(vec![]),
        });

        let table = Rc::new(Furniture {
            id: String::from("table1"),
            description: String::from("A wooden table"),
            house: Rc::downgrade(&house1), // downgrade goes with Weak refs
        });

        let desk = Rc::new(Furniture {
            id: String::from("desk1"),
            description: String::from("A wooden desk"),
            house: Rc::downgrade(&house1),
        });

        house1.furniture.borrow_mut().push(Rc::clone(&table));
        house1.furniture.borrow_mut().push(Rc::clone(&desk));

        dbg!(&house1);
    }
}
