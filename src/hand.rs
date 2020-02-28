use super::card::*;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
pub enum HandRank {
    HighCard(Value, Value, Value, Value, Value),
    Pair(Value, Value, Value, Value),
    TwoPairs(Value, Value, Value),
    Trips(Value, Value, Value),
    Straight(Value),
    Flush(Value, Value, Value, Value, Value),
    FullHouse(Value, Value),
    Quads(Value, Value),
    StraightFlush(Value),
}

impl From<[Card; 5]> for HandRank {
    fn from(cards: [Card; 5]) -> HandRank {
        use itertools::Itertools;
        use Value::*;

        // Flush
        let mut s: Vec<_> = cards.iter().map(|x| x.0).collect();
        let s0 = s.pop().unwrap();
        let is_flush = s.into_iter().all(|x| x == s0);

        // Straight
        let mut v: Vec<_> = cards.iter().map(|x| x.1).collect();
        v.sort();
        v.reverse();
        let is_straight_normal = v
            .iter()
            .tuple_windows()
            .all(|(x, y)| i8::from(*x) == i8::from(*y) + 1);
        let is_straight_ace = v == vec![Ace, Five, Four, Three, Two];

        if is_flush {
            if is_straight_normal {
                return HandRank::StraightFlush(v[0]);
            } else if is_straight_ace {
                return HandRank::StraightFlush(Five);
            } else {
                return HandRank::Flush(v[0], v[1], v[2], v[3], v[4]);
            }
        }

        if is_straight_normal {
            return HandRank::Straight(v[0]);
        } else if is_straight_ace {
            return HandRank::Straight(Five);
        }

        let mut v = v;
        let mut counter = vec![(1, v.pop().unwrap())];
        while let Some(value) = v.pop() {
            let mut x = counter.pop().unwrap();
            if x.1 == value {
                x.0 += 1;
                counter.push(x);
            } else {
                counter.push(x);
                counter.push((1, value));
            }
        }

        counter.sort();
        counter.reverse();

        let c = counter.iter().map(|x| x.0).next_tuple().unwrap();
        let v: Vec<_> = counter.iter().map(|x| x.1).collect();

        match c {
            (4, 1) => HandRank::Quads(v[0], v[1]),
            (3, 2) => HandRank::FullHouse(v[0], v[1]),
            (3, 1) => HandRank::Trips(v[0], v[1], v[2]),
            (2, 2) => HandRank::TwoPairs(v[0], v[1], v[2]),
            (2, 1) => HandRank::Pair(v[0], v[1], v[2], v[3]),
            (1, 1) => HandRank::HighCard(v[0], v[1], v[2], v[3], v[4]),
            _ => unreachable!(),
        }
    }
}

impl From<[Card; 7]> for HandRank {
    fn from(cards: [Card; 7]) -> HandRank {
        let mut v = vec![];
        for i in 0..7 {
            for j in 0..i {
                for k in 0..j {
                    for l in 0..k {
                        for m in 0..l {
                            v.push(HandRank::from([
                                cards[i], cards[j], cards[k], cards[l], cards[m],
                            ]))
                        }
                    }
                }
            }
        }
        v.into_iter().max().unwrap()
    }
}
