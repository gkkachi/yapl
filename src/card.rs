use std::fmt;

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

impl fmt::Debug for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Suit::*;
        match self {
            Spades => write!(f, "♠"),
            Hearts => write!(f, "♥"),
            Diamonds => write!(f, "♦"),
            Clubs => write!(f, "♣"),
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
pub enum Value {
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
    Ace,
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Value::*;
        match self {
            Ten => write!(f, "T"),
            Jack => write!(f, "J"),
            Queen => write!(f, "Q"),
            King => write!(f, "K"),
            Ace => write!(f, "A"),
            x => write!(f, "{}", i8::from(*x)),
        }
    }
}

impl From<Value> for i8 {
    fn from(v: Value) -> i8 {
        use Value::*;
        match v {
            Two => 2,
            Three => 3,
            Four => 4,
            Five => 5,
            Six => 6,
            Seven => 7,
            Eight => 8,
            Nine => 9,
            Ten => 10,
            Jack => 11,
            Queen => 12,
            King => 13,
            Ace => 14,
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
pub struct Card(pub Suit, pub Value);

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}{:?}", self.1, self.0)
    }
}
