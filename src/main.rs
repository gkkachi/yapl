use std::cmp::Ordering;
use yapl::{deck::*, hand::*};

fn main() {
    let mut d = Deck::new().into_iter();
    let x0 = d.next().unwrap();
    let x1 = d.next().unwrap();
    let y0 = d.next().unwrap();
    let y1 = d.next().unwrap();
    let c0 = d.next().unwrap();
    let c1 = d.next().unwrap();
    let c2 = d.next().unwrap();
    let c3 = d.next().unwrap();
    let c4 = d.next().unwrap();
    let x = [x0, x1, c0, c1, c2, c3, c4];
    let y = [y0, y1, c0, c1, c2, c3, c4];

    let rank_x: HandRank = x.into();
    let rank_y: HandRank = y.into();

    println!("Common Cards:\n{:?}\n", [c0, c1, c2, c3, c4]);
    println!("Player 1:\n{:?}\n{:?}\n", [x0, x1], rank_x);
    println!("Player 2:\n{:?}\n{:?}\n", [y0, y1], rank_y);

    match rank_x.cmp(&rank_y) {
        Ordering::Equal => println!("TIE"),
        Ordering::Greater => println!("Player 1 wins."),
        Ordering::Less => println!("Player 2 wins."),
    };
}
