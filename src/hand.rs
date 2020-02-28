use super::card::*;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
pub enum HandRank {
    HighCard([Value; 5]),
    Pair(Value, [Value; 3]),
    TwoPairs([Value; 2], Value),
    Trips(Value, [Value; 2]),
    Straight(Value),
    Flush([Value; 5]),
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
        let v: Vec<_> = cards.iter().map(|x| x.1).sorted().collect();
        let is_straight_normal = v
            .iter()
            .tuple_windows()
            .all(|(x, y)| i8::from(*x) + 1 == i8::from(*y));
        let is_straight_ace = vec![Ace, Two, Three, Four, Five] == v;
        let is_straight = is_straight_ace || is_straight_normal;

        if is_flush {
            if is_straight {
                let mut v = v;
                return HandRank::StraightFlush(v.pop().unwrap());
            } else {
                let mut v = v;
                return HandRank::Flush([
                    v.pop().unwrap(),
                    v.pop().unwrap(),
                    v.pop().unwrap(),
                    v.pop().unwrap(),
                    v.pop().unwrap(),
                ]);
            }
        }

        if is_straight {
            let mut v = v;
            return HandRank::Straight(v.pop().unwrap());
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

        match counter.pop().unwrap() {
            (4, x) => HandRank::Quads(x, counter.pop().unwrap().1),
            (3, x) => match counter.len() {
                1 => HandRank::FullHouse(x, counter.pop().unwrap().1),
                2 => HandRank::Trips(x, [counter.pop().unwrap().1, counter.pop().unwrap().1]),
                _ => unreachable!(),
            },
            (2, x) => match counter.len() {
                2 => HandRank::TwoPairs([x, counter.pop().unwrap().1], counter.pop().unwrap().1),
                3 => HandRank::Pair(
                    x,
                    [
                        counter.pop().unwrap().1,
                        counter.pop().unwrap().1,
                        counter.pop().unwrap().1,
                    ],
                ),
                _ => unreachable!(),
            },
            (1, x) => HandRank::HighCard([
                x,
                counter.pop().unwrap().1,
                counter.pop().unwrap().1,
                counter.pop().unwrap().1,
                counter.pop().unwrap().1,
            ]),
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
