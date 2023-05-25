use std::fmt;
#[derive(Clone, Debug, Eq, PartialEq)]

pub enum Value {
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
    JokerRed,
    JokerBlack,
}

impl From<char> for Value {
    fn from(input: char) -> Self {
        match input {
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
            'B' => Value::JokerBlack,
            'R' => Value::JokerRed,
            _ => panic!("Invalid card value {}", input),
        }
    }
}

impl From<Value> for usize {
    fn from(value: Value) -> usize {
        match value {
            Value::Ace => 1,
            Value::Two => 2,
            Value::Three => 3,
            Value::Four => 4,
            Value::Five => 5,
            Value::Six => 6,
            Value::Seven => 7,
            Value::Eight => 8,
            Value::Nine => 9,
            Value::Ten => 10,
            Value::Jack => 11,
            Value::Queen => 12,
            Value::King => 13,
            Value::JokerBlack => 53,
            Value::JokerRed => 54,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display = match self {
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
            Value::JokerBlack => 'B',
            Value::JokerRed => 'R',
        };
        write!(f, "{}", display)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]

pub enum Suit {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
    Joker,
}

impl From<char> for Suit {
    fn from(value: char) -> Self {
        match value {
            'S' => Suit::Spades,
            'H' => Suit::Hearts,
            'C' => Suit::Clubs,
            'D' => Suit::Diamonds,
            'J' => Suit::Joker,
            _ => panic!("Invalid suit value {}", value),
        }
    }
}

impl From<Suit> for usize {
    fn from(value: Suit) -> Self {
        match value {
            Suit::Joker => 0,
            Suit::Clubs => 0,
            Suit::Diamonds => 13,
            Suit::Hearts => 26,
            Suit::Spades => 39,
        }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display = match self {
            Suit::Spades => 'S',
            Suit::Hearts => 'H',
            Suit::Clubs => 'C',
            Suit::Diamonds => 'D',
            Suit::Joker => 'J',
        };
        write!(f, "{}", display)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Card {
    value: Value,
    suit: Suit,
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        let mut chars = value.chars();
        let value_char = chars.next().unwrap();
        let suit_char = chars.next().unwrap();

        Card {
            value: Value::from(value_char),
            suit: Suit::from(suit_char),
        }
    }
}

impl From<Card> for usize {
    fn from(value: Card) -> Self {
        let val: usize = value.value.into();
        let suit: usize = value.suit.into();

        val + suit
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.value, self.suit)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Deck {
    // TODO: circular linked list
    pub cards: Vec<Card>,
    pub joker_red: usize,
    pub joker_black: usize,
}

impl Deck {
    // TODO: generate deck from default and key for shuffling
    pub fn new() -> Self {
        Self {
            cards: Vec::new(),
            joker_red: 0,
            joker_black: 0,
        }
    }

    pub fn find_joker(&mut self, joker: char) -> usize {
        match joker {
            'R' => self
                .cards
                .iter()
                .position(|card| card.value == Value::JokerRed)
                .unwrap(),
            _ => self
                .cards
                .iter()
                .position(|card| card.value == Value::JokerBlack)
                .unwrap(),
        }
    }

    pub fn move_joker_red(&mut self) {
        if self.joker_red == self.cards.len() - 1 {
            let card = self.cards.pop().unwrap();
            self.cards.insert(1, card);
        } else {
            self.cards.swap(self.joker_red, self.joker_red + 1);

            self.joker_red = self.find_joker('R');
            self.joker_black = self.find_joker('B');
        }
    }

    pub fn move_joker_black(&mut self) {
        if self.joker_black == self.cards.len() - 2 {
            let card = self.cards.remove(self.cards.len() - 2);
            self.cards.insert(1, card);
        } else if self.joker_black == self.cards.len() - 1 {
            let card = self.cards.pop().unwrap();
            self.cards.insert(2, card);
        } else {
            let card = self.cards.remove(self.joker_black);
            self.cards.insert(self.joker_black + 2, card)
        }

        self.joker_red = self.find_joker('R');
        self.joker_black = self.find_joker('B');
    }

    pub fn triple_cut(&mut self) {
        let bottom_joker = std::cmp::min(self.joker_red, self.joker_black);
        let top_joker = std::cmp::max(self.joker_red, self.joker_black);

        let mut cards: Vec<Card> = Vec::with_capacity(54);
        if top_joker < self.cards.len() - 1 {
            cards.extend(self.cards[top_joker + 1..].iter().cloned());
        }
        cards.extend(self.cards[bottom_joker..top_joker + 1].iter().cloned());
        cards.extend(self.cards[..bottom_joker].iter().cloned());

        let joker_red = self.find_joker('R');
        let joker_black = self.find_joker('B');

        *self = Deck {
            cards,
            joker_red,
            joker_black,
        };
    }

    pub fn count_cut(&mut self) {
        let mut cards = self.cards.clone();
        let last_card = cards.pop().unwrap();
        let last_card_value: usize = last_card.clone().into();

        let mut cut_deck: Vec<Card> = cards.drain(..last_card_value).collect();
        cards.append(&mut cut_deck);
        cards.push(last_card);

        let joker_red = self.find_joker('R');
        let joker_black = self.find_joker('B');

        *self = Deck {
            cards,
            joker_red,
            joker_black,
        };
    }

    pub fn get_output_value(&self) -> Result<u8, ()> {
        let count_val: usize = match self.cards[0].clone().into() {
            53 | 54 => 53,
            v => v,
        };

        let output_val: usize = self.cards[count_val].clone().into();

        if output_val > 52 {
            Err(())
        } else {
            Ok(output_val as u8)
        }
    }

    pub fn keystream(&mut self) -> u8 {
        let output: u8;

        loop {
            self.move_joker_red();
            self.move_joker_black();
            self.triple_cut();
            self.count_cut();

            match self.get_output_value() {
                Ok(c) => {
                    output = c;
                    break;
                }
                Err(_) => continue,
            }
        }

        output
    }

    pub fn encrypt(&mut self, plaintext: &str) -> String {
        let mut output = String::new();
        for c in plaintext.to_ascii_uppercase().chars() {
            if c != ' ' {
                let key = self.keystream();
                let mut ciphertext = key + (c as u8 - 64);

                while ciphertext > 26 {
                    ciphertext -= 26
                }

                output.push(char::from(ciphertext + 64));
            }
        }

        output
    }

    pub fn decrypt(&mut self, ciphertext: &str) -> String {
        let mut output = String::new();

        for c in ciphertext.to_ascii_uppercase().chars() {
            let key = self.keystream();
            let mut plaintext = (c as i8 - 64) - key as i8;

            while plaintext < 1 {
                plaintext += 26;
            }

            output.push(char::from(plaintext as u8 + 64))
        }

        output
    }
}

impl From<&str> for Deck {
    fn from(value: &str) -> Self {
        let mut deck = Self::new();
        for (idx, card_string) in value.trim().split_whitespace().enumerate() {
            let card = Card::from(card_string);
            match card.value {
                Value::JokerBlack => deck.joker_black = idx,
                Value::JokerRed => deck.joker_red = idx,
                _ => (),
            }
            deck.cards.push(card);
        }
        deck
    }
}

impl Default for Deck {
    fn default() -> Self {
        let input = "AC 2C 3C 4C 5C 6C 7C 8C 9C TC JC QC KC AD 2D 3D 4D 5D 6D 7D 8D 9D TD JD QD KD AH 2H 3H 4H 5H 6H 7H 8H 9H TH JH QH KH AS 2S 3S 4S 5S 6S 7S 8S 9S TS JS QS KS BJ RJ";
        Deck::from(input)
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut display = String::new();
        for card in self.cards.iter() {
            display.push_str(&format!("{} ", *card))
        }

        write!(f, "{}", display)
    }
}

fn main() {
    let mut d = Deck::default();
    let mut d1 = Deck::default();

    let plaintext = "DO NOT USE PC";
    let ct = d.encrypt(plaintext);
    println!("{}", ct);

    println!("{}", d1.decrypt(ct.as_str()))
}
