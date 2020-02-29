use super::card::*;
use std::vec::IntoIter;

pub struct Deck(Vec<Card>);

impl Deck {
    pub fn new() -> Self {
        use itertools::Itertools;
        use Suit::*;
        use Value::*;

        let s = [Spades, Hearts, Diamonds, Clubs].iter();
        let v = [
            Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace,
        ]
        .iter();

        let cards = s.cartesian_product(v).map(|x| Card(*x.0, *x.1)).collect();
        Deck(cards)
    }

    pub fn shuffled(self) -> Self {
        use rand::{seq::SliceRandom, thread_rng};
        let Deck(mut cards) = self;
        cards.shuffle(&mut thread_rng());
        Deck(cards)
    }

    pub fn remove_card(&mut self, card: &Card) {
        let Deck(ref mut cards) = self;
        if let Some(pos) = cards.iter().position(|x| *x == *card) {
            cards.remove(pos);
        }
    }

    pub fn all_patterns(self, n: usize) -> impl Iterator<Item = Vec<Card>> {
        let m = self.0.len();
        let Deck(cards) = self;
        indices(m, n)
            .into_iter()
            .map(move |x| x.into_iter().map(|v| cards[v]).collect::<Vec<_>>())
    }
}

impl IntoIterator for Deck {
    type Item = Card;
    type IntoIter = IntoIter<Card>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

fn indices(m: usize, n: usize) -> Vec<Vec<usize>> {
    assert!(m >= n);
    if n == 0 {
        vec![vec![]]
    } else {
        let n = n - 1;
        (n..m)
            .map(|x| {
                indices(x, n).into_iter().map(move |mut v| {
                    v.push(x);
                    v
                })
            })
            .flatten()
            .collect()
    }
}
