use std::cell::RefCell;
use std::fmt;
use std::rc::{Rc, Weak};

enum Value {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Joker,
}

enum Suit {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
}

struct CardData {
    value: Value,
    suit: Suit,
}

impl CardData {
    pub fn string_to_carddata(input: &str) -> Option<CardData> {
        if input == "BJ" {
            return Some(CardData {
                value: Value::Joker,
                suit: Suit::Spades,
            });
        } else if input == "RJ" {
            return Some(CardData {
                value: Value::Joker,
                suit: Suit::Hearts,
            });
        }

        let mut chars = input.chars();
        let value_char = chars.next()?;
        let suit_char = chars.next()?;

        let value = match value_char {
            'A' => Value::Ace,
            '2' => Value::Two,
            '3' => Value::Three,
            '4' => Value::Four,
            '5' => Value::Five,
            '6' => Value::Six,
            '7' => Value::Seven,
            '8' => Value::Eight,
            '9' => Value::Nine,
            'T' => Value::Ten,
            'J' => Value::Jack,
            'Q' => Value::Queen,
            'K' => Value::King,
            _ => return None, // invalid value char
        };

        let suit = match suit_char {
            'S' => Suit::Spades,
            'H' => Suit::Hearts,
            'C' => Suit::Clubs,
            'D' => Suit::Diamonds,
            _ => return None, // invalid suit char
        };

        Some(CardData { value, suit })
    }

    // pub fn string_to_cards(input: &str) -> Result<Vec<Card>, String> {
    //     let cards_strings: Vec<&str> = input.split_whitespace().collect();
    //     let mut cards = Vec::with_capacity(cards_strings.len());

    //     for card_string in cards_strings {
    //         let card = Self::string_to_card(card_string)?;
    //         cards.push(card);
    //     }

    //     if Self::valid_deck(cards) {
    //         return cards;
    //     }
    //     Err("Invalid deck".to_string())
    // }
}

struct Card {
    pub data: CardData,
    pub prev: Option<Weak<RefCell<Card>>>,
    pub next: Option<Rc<RefCell<Card>>>,
}

impl Card {
    pub fn new(data: CardData) -> Self {
        Self {
            data,
            prev: None,
            next: None,
        }
    }

    pub fn append(node: &mut Rc<RefCell<Card>>, data: CardData) -> Option<Rc<RefCell<Card>>> {
        let is_last = node.borrow().next.is_none();
        if is_last {
            let mut new_node = Card::new(data);
            new_node.prev = Some(Rc::downgrade(&node));
            let rc = Rc::new(RefCell::new(new_node));
            node.borrow_mut().next = Some(rc.clone());
            Some(rc)
        } else {
            if let Some(ref mut next) = node.borrow_mut().next {
                Self::append(next, data)
            } else {
                None
            }
        }
    }
}

struct Deck {
    first: Option<Rc<RefCell<Card>>>,
    last: Option<Rc<RefCell<Card>>>,
}

impl Deck {
    pub fn new() -> Self {
        Self {
            first: None,
            last: None,
        }
    }

    pub fn append(&mut self, data: &str) {
        let carddata;
        match CardData::string_to_carddata(data) {
            Some(data) => carddata = data,
            None => panic!("{} invalid card", data),
        }

        if let Some(ref mut next) = self.first {
            self.last = Card::append(next, carddata);
        } else {
            let f = Rc::new(RefCell::new(Card::new(carddata)));
            self.first = Some(f.clone());
            self.last = Some(f);
        }
    }

    pub fn from_string(input: &str) -> Self {
        let cards_strings: Vec<&str> = input.trim().split_whitespace().collect();
        let mut deck = Self::new();
        for card_string in cards_strings {
            deck.append(card_string);
        }

        deck
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut cards = vec![];
        let mut current = self.first.clone();

        while let Some(node) = current {
            let card = node.borrow();
            cards.push(format!("{}", card));
            current = card.next.clone();
        }

        write!(f, "{}", cards.join(", "))
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value_char = match self.data.value {
            Value::Ace => 'A',
            Value::Two => '2',
            Value::Three => '3',
            Value::Four => '4',
            Value::Five => '5',
            Value::Six => '6',
            Value::Seven => '7',
            Value::Eight => '8',
            Value::Nine => '9',
            Value::Ten => 'T',
            Value::Jack => 'J',
            Value::Queen => 'Q',
            Value::King => 'K',
            Value::Joker => '?',
        };

        let suit_char = match self.data.suit {
            Suit::Spades => 'S',
            Suit::Hearts => 'H',
            Suit::Clubs => 'C',
            Suit::Diamonds => 'D',
        };

        write!(f, "{}{}", value_char, suit_char)
    }
}

fn main() {
    let list = Deck::from_string(" AS AH 3H");

    println!("{}", list);
}
