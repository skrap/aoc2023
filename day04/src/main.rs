use std::collections::HashMap;

struct Card {
    winners: Vec<i32>,
    have: Vec<i32>,
}

fn parse(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|line| {
            let (_card, rest) = line.split_once(':').unwrap();
            let (winners, have) = rest.trim().split_once(" | ").unwrap();
            Card {
                winners: winners
                    .trim()
                    .split_whitespace()
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect(),
                have: have
                    .trim()
                    .split_whitespace()
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect(),
            }
        })
        .collect()
}

impl Card {
    fn winner_cnt(&self) -> usize {
        self.have
            .iter()
            .filter(|n| self.winners.contains(n))
            .count()
    }

    fn score(&self) -> i32 {
        match self.winner_cnt() {
            0 => 0,
            n => 1 << (n - 1),
        }
    }
}

fn part1(input: &str) -> i32 {
    let cards = parse(input);
    cards.iter().map(Card::score).sum()
}

fn part2(input: &str) -> i32 {
    let cards = parse(input);
    let mut memo = HashMap::new();
    fn card_cnt(idx: usize, cards: &[Card], memo: &mut HashMap<usize, i32>) -> i32 {
        if let Some(n) = memo.get(&idx) {
            *n
        } else if let Some(card) = cards.get(idx) {
            let winnings = idx + 1..idx + 1 + card.winner_cnt();
            let result = 1 + winnings.map(|idx| card_cnt(idx, cards, memo)).sum::<i32>();
            memo.insert(idx, result);
            result
        } else {
            0
        }
    }
    (0..cards.len()).map(|n| card_cnt(n, &cards, &mut memo)).sum()
}

fn main() {
    let input = include_str!("../input");
    dbg!(part1(input));
    dbg!(part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
        assert_eq!(30, part2(input));
    }
}
