use rand::{seq::SliceRandom, thread_rng};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
    author: String,
}

impl Deck {
    fn new() -> Self {
        //List of suits: hearts, spades, clubs, diamonds
        let suits = ["hearts".to_string(), "spades".to_string()];
        //List of values ace, two, ...
        let values = ["ace".to_string(), "two".to_string()];

        let mut cards = Vec::new();

        // nested loop
        for suit in suits {
            for value in values.clone() {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        Deck {
            cards,
            author: "Zake".to_string(),
        }
    }

    fn shuffle(&mut self) {
        println!("{:?}", self);
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num)
    }
}

fn main() {
    let mut deck: Deck = Deck::new();
    deck.shuffle();

    println!("Here's the deck: {:?}", deck);

    let new_deck = deck.deal(2);

    println!("Here's the new deck: {:?}", new_deck);
}
