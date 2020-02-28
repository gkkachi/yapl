use super::card::*;
use std::vec::IntoIter;

pub struct Deck(Vec<Card>);

impl Deck {
    pub fn new() -> Self {
        use itertools::Itertools;
        use rand::{seq::SliceRandom, thread_rng};
        use Suit::*;
        use Value::*;

        let s = [Spades, Hearts, Diamonds, Clubs].iter();
        let v = [
            Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace,
        ]
        .iter();

        let mut cards: Vec<_> = s.cartesian_product(v).map(|x| Card(*x.0, *x.1)).collect();
        cards.shuffle(&mut thread_rng());
        Deck(cards)
    }
}

impl IntoIterator for Deck {
    type Item = Card;
    type IntoIter = IntoIter<Card>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
