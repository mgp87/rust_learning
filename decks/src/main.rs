use core::num;

use rand::{rng, seq::SliceRandom};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    // Associated functions, tied to the struct definition --> Deck::new()
    fn new() -> Self { // Returns Deck type
        // List of 'suits' - 'hearts', 'spades'...
        let suits = ["Hearts", "Spades", "Diamonds", "Clubs"]; // Array: it can't grow or shrink

        // List of 'values' - 'ace', 'two'...
        let values = ["Ace", "Two", "Three"];

        let mut cards = vec![];

        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        Deck{cards} // cards: cards --> Implicit return (no ;)
    }

    // Method, operates on specific instance of a struct (&self) --> instance.shuffle()
    fn shuffle(&mut self) {
        let mut rng = rng();
        self.cards.shuffle(&mut rng); // SliceRandom function
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(
            self.cards.len() - num_cards
        )
    }
}

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();
    let cards = deck.deal(3);
    println!("Heres the deck: {:#?}", deck);
    println!("Heres the deal: {:#?}", cards);
}
