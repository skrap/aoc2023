fn main() {
    let input = include_str!("../input");
    dbg!(part1(input));
    dbg!(part2(input));
}

fn part1(input: &str) -> usize {
    let mut hands = parse(input);
    hands.sort_by_key(Hand::sortkey_part1);
    hands
        .iter()
        .zip(1..)
        .map(|(hand, rank)| hand.bid * rank)
        .sum()
}

fn part2(input: &str) -> usize {
    let mut hands = parse(input);
    hands.sort_by_key(Hand::sortkey_part2);
    hands
        .iter()
        .zip(1..)
        .map(|(hand, rank)| hand.bid * rank)
        .sum()
}

fn parse(input: &str) -> Vec<Hand> {
    input
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(" ").unwrap();
            let cards: Vec<_> = cards.chars().map(to_card).collect();
            Hand {
                cards: cards.try_into().unwrap(),
                bid: bid.parse().unwrap(),
            }
        })
        .collect()
}

#[derive(Debug)]
struct Hand {
    cards: [u8; 5],
    bid: usize,
}

impl Hand {
    fn sortkey_part1(&self) -> usize {
        let mut counts = [0u8; 15];
        for card in self.cards {
            counts[card as usize] += 1;
        }
        counts.sort();
        counts.reverse();
        let htype = match counts[0] {
            1 => 1,                  // high card,
            2 if counts[1] < 2 => 2, // one pair
            2 => 3,                  // two pair
            3 if counts[1] < 2 => 4, // three of kind
            3 => 5,                  // full house
            4 => 6,                  // four of a kind
            5 => 7,                  // five of a kind
            _ => unimplemented!(),
        };
        let mut result: usize = htype;
        for card in self.cards {
            result <<= 8;
            result += card as usize;
        }
        result
    }

    fn sortkey_part2(&self) -> usize {
        let mut counts = [0u8; 15];
        let mut wilds = 0;
        for card in self.cards {
            match card {
                11 => wilds += 1,
                n => counts[n as usize] += 1,
            }
        }
        counts.sort();
        counts.reverse();
        counts[0] += wilds;
        let htype = match counts[0] {
            1 => 1,                  // high card,
            2 if counts[1] < 2 => 2, // one pair
            2 => 3,                  // two pair
            3 if counts[1] < 2 => 4, // three of kind
            3 => 5,                  // full house
            4 => 6,                  // four of a kind
            5 => 7,                  // five of a kind
            _ => unimplemented!(),
        };

        let mut result: usize = htype;
        for card in self.cards {
            result <<= 8;
            result += match card {
                11 => 1,
                n => n,
            } as usize;
        }
        result
    }
}

fn to_card(c: char) -> u8 {
    if let Some(n) = c.to_digit(10) {
        if n >= 2 {
            return n.try_into().unwrap();
        }
    }
    match c {
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(6440, part1(input));
        assert_eq!(5905, part2(input));
    }
}
