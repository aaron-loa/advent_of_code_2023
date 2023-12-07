use std::{cmp::Ordering, str::FromStr};

#[derive(Debug, Ord, PartialEq, PartialOrd, Eq, Clone)]
#[repr(usize)]
enum Card {
    CJ = 2,
    C2 = 3,
    C3 = 4,
    C4 = 5,
    C5 = 6,
    C6 = 7,
    C7 = 8,
    C8 = 9,
    C9 = 10,
    CT = 11,
    CQ = 12,
    CK = 13,
    CA = 14,
}

impl From<Card> for usize {
    fn from(card: Card) -> Self {
        match card {
            Card::CJ => 2,
            Card::C2 => 3,
            Card::C3 => 4,
            Card::C4 => 5,
            Card::C5 => 6,
            Card::C6 => 7,
            Card::C7 => 8,
            Card::C8 => 9,
            Card::C9 => 10,
            Card::CT => 11,
            Card::CQ => 12,
            Card::CK => 13,
            Card::CA => 14,
        }
    }
}

impl From<&Card> for usize {
    fn from(card: &Card) -> Self {
        match card {
            Card::CJ => 2,
            Card::C2 => 3,
            Card::C3 => 4,
            Card::C4 => 5,
            Card::C5 => 6,
            Card::C6 => 7,
            Card::C7 => 8,
            Card::C8 => 9,
            Card::C9 => 10,
            Card::CT => 11,
            Card::CQ => 12,
            Card::CK => 13,
            Card::CA => 14,
        }
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    value: usize,
    score: usize,
}
impl Hand {
    fn grade(&mut self) {
        let mut counters = [0usize; 15];

        self.cards.iter().for_each(|card| {
            counters[usize::from(card)] += 1;
        });
        let jokers = counters[2];
        let mut max_pos = 0;
        let mut max = 0;
        if jokers != 0 {
            for (idx, value) in counters.iter().enumerate() {
                if *value >= max {
                    max = *value;
                    max_pos = idx;
                }
            }
            if max_pos == 2 {
                max_pos = 0;
                max = 0;
                for (idx, value) in counters.iter().enumerate().skip(3) {
                    if *value >= max {
                        max = *value;
                        max_pos = idx;
                    }
                }
                counters[max_pos] += jokers;
                counters[2] = 0;
            } else {
                counters[max_pos] += jokers;
                counters[2] = 0;
            }
        }

        let pairs = counters.iter().filter(|x| **x == 2).count();
        let triples = counters.iter().filter(|x| **x == 3).count();
        let quads = counters.iter().filter(|x| **x == 4).count();
        let five = counters.iter().filter(|x| **x == 5).count();

        if five == 1 {
            self.score = 1000000000;
            return;
        }
        if quads == 1 {
            self.score = 100000000;
            return;
        }
        if triples == 1 && pairs == 1 {
            self.score = 10000000;
            return;
        }
        if triples == 1 {
            self.score = 1000000;
            return;
        }
        if pairs == 2 {
            self.score = 100000;
            return;
        }
        if pairs == 1 {
            self.score = 1000;
            return;
        }
        self.score = 1;
        return;
    }
}

impl FromStr for Hand {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand, value) = s.split_once(" ").unwrap();
        let value = value.parse::<usize>().unwrap();
        let vector: Vec<Card> = hand
            .chars()
            .map(|c| match c {
                '2' => Ok(Card::C2),
                '3' => Ok(Card::C3),
                '4' => Ok(Card::C4),
                '5' => Ok(Card::C5),
                '6' => Ok(Card::C6),
                '7' => Ok(Card::C7),
                '8' => Ok(Card::C8),
                '9' => Ok(Card::C9),
                'T' => Ok(Card::CT),
                'J' => Ok(Card::CJ),
                'Q' => Ok(Card::CQ),
                'K' => Ok(Card::CK),
                'A' => Ok(Card::CA),
                _ => Err(()),
            })
            .map(|x| x.unwrap())
            .collect();
        Ok(Hand {
            cards: vector,
            value,
            score: 0,
        })
    }
}

fn part2() {
    let input = include_str!("input");
    let mut res: Vec<_> = input
        .lines()
        .map(|line| {
            let mut hand = Hand::from_str(line).unwrap();
            hand.grade();
            hand
        })
        .collect();
    res.sort_by(|a, b| {
        if a.score == b.score {
            a.cards
                .iter()
                .zip(b.cards.iter())
                .find_map(|(first, second)| {
                    if first == second {
                        return None;
                    };
                    return Some(first.cmp(second));
                })
                .unwrap_or(Ordering::Equal)
        } else {
            a.score.cmp(&b.score)
        }
    });

    let sum: usize = res
        .iter()
        .enumerate()
        .map(|(idx, hand)| (idx + 1) * hand.value)
        .sum();
    println!("{}", sum);
}

fn main() {
    let start = std::time::Instant::now();
    part2();
    let end = std::time::Instant::now();
    println!("{:?}", end - start);
}
