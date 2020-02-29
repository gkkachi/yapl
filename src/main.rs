use std::cmp::Ordering;
use yapl::{deck::*, hand::*};

fn main() {
    let mut d = Deck::new().shuffled().into_iter();
    let x0 = d.next().unwrap();
    let x1 = d.next().unwrap();
    let y0 = d.next().unwrap();
    let y1 = d.next().unwrap();
    let c0 = d.next().unwrap();
    let c1 = d.next().unwrap();
    let c2 = d.next().unwrap();

    let mut d = Deck::new();
    [x0, x1, y0, y1, c0, c1, c2]
        .iter()
        .for_each(|c| d.remove_card(c));

    let res: [_; 3] = d
        .all_patterns(2)
        .map(|cards| {
            let c3 = cards[0];
            let c4 = cards[1];
            let x = [x0, x1, c0, c1, c2, c3, c4];
            let y = [y0, y1, c0, c1, c2, c3, c4];

            let rank_x: HandRank = x.into();
            let rank_y: HandRank = y.into();

            match rank_x.cmp(&rank_y) {
                Ordering::Equal => [0, 0, 1],
                Ordering::Greater => [1, 0, 0],
                Ordering::Less => [0, 1, 0],
            }
        })
        .fold([0, 0, 0], |x, y| [x[0] + y[0], x[1] + y[1], x[2] + y[2]]);

    println!(
        "Community cards: {:?}\nPlayer 1: {:?}\nPlayer 2: {:?}\n{:?}",
        [c0, c1, c2],
        [x0, x1],
        [y0, y1],
        res
    );
}
