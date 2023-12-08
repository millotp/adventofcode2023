use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("inputs/day7.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

struct Hand {
    cards: [char; 5],
}

impl Hand {
    fn new(cards: [char; 5]) -> Hand {
        Hand { cards }
    }

    fn combi1(&self) -> u64 {
        let mut occu = HashMap::new();
        for c in self.cards.iter() {
            let count = occu.entry(c).or_insert(0);
            *count += 1;
        }

        let mut counts = occu.values().map(|x| *x).collect::<Vec<_>>();
        counts.sort();

        if counts.len() == 1 {
            return 7;
        }
        if counts.len() == 2 {
            if counts[0] == 1 {
                return 6;
            }
            return 5;
        }
        if counts.len() == 3 {
            if counts[2] == 3 {
                return 4;
            }
            return 3;
        }
        if counts.len() == 4 {
            return 2;
        }

        return 1;
    }

    fn combi2(&self) -> u64 {
        let mut occu = HashMap::new();
        for c in self.cards.iter() {
            let count = occu.entry(c).or_insert(0);
            *count += 1;
        }

        let mut counts = occu.values().map(|x| *x).collect::<Vec<_>>();
        counts.sort();

        if counts.len() == 1 {
            // 5 of a kind
            return 7;
        }
        if counts.len() == 2 {
            // 4 of a kind or full house
            if self.cards.iter().find(|x| **x == 'J').is_some() {
                return 7;
            }
            if counts[0] == 1 {
                return 6;
            }
            return 5;
        }
        if counts.len() == 3 {
            if counts[2] == 3 {
                // Three of a kind
                if self.cards.iter().find(|x| **x == 'J').is_some() {
                    return 6;
                }
                return 4;
            }

            // Two pairs
            let j_count = self.cards.iter().filter(|x| **x == 'J').count();
            if j_count == 2 {
                return 6;
            }
            if j_count == 1 {
                return 5;
            }
            return 3;
        }
        if counts.len() == 4 {
            // One pair
            if self.cards.iter().find(|x| **x == 'J').is_some() {
                return 4;
            }
            return 2;
        }

        if self.cards.iter().find(|x| **x == 'J').is_some() {
            return 2;
        }

        return 1;
    }
}

fn part1(input: &str) -> u64 {
    let order = "23456789TJQKA";
    let mut hands = input
        .lines()
        .map(|line| {
            let (hand, score) = line.split_once(" ").unwrap();
            let hand = Hand::new(hand.chars().collect::<Vec<_>>().try_into().unwrap());

            (hand.cards, hand.combi1(), score.parse::<u64>().unwrap())
        })
        .collect::<Vec<_>>();

    hands.sort_by(|a, b| {
        if a.1 == b.1 {
            for i in 0..5 {
                if a.0[i] != b.0[i] {
                    return order
                        .find(a.0[i])
                        .unwrap()
                        .cmp(&order.find(b.0[i]).unwrap());
                }
            }
        }

        return a.1.cmp(&b.1);
    });

    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, x)| acc + x.2 * (i + 1) as u64)
}

fn part2(input: &str) -> u64 {
    let order = "J23456789TQKA";
    let mut hands = input
        .lines()
        .map(|line| {
            let (hand, score) = line.split_once(" ").unwrap();
            let hand = Hand::new(hand.chars().collect::<Vec<_>>().try_into().unwrap());

            (hand.cards, hand.combi2(), score.parse::<u64>().unwrap())
        })
        .collect::<Vec<_>>();

    hands.sort_by(|a, b| {
        if a.1 == b.1 {
            for i in 0..5 {
                if a.0[i] != b.0[i] {
                    return order
                        .find(a.0[i])
                        .unwrap()
                        .cmp(&order.find(b.0[i]).unwrap());
                }
            }
        }

        return a.1.cmp(&b.1);
    });

    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, x)| acc + x.2 * (i + 1) as u64)
}
