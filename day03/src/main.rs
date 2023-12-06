use std::{collections::HashMap, ops::Add};

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
struct Pt(i32, i32);

impl Add for Pt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Pt(self.0 + rhs.0, self.1 + rhs.1)
    }
}

struct Schm {
    nums: Vec<(Pt, i32)>,
    syms: HashMap<Pt, char>,
}

fn parse(input: &str) -> Schm {
    let mut nums = vec![];
    let mut syms = HashMap::new();

    let mut pt = Pt(0, 0);
    let mut num = None;
    for b in input.bytes() {
        if b.is_ascii_digit() {
            num = num
                .map(|(pt, n)| (pt, 10 * n + (b - b'0') as i32))
                .or(Some((pt, (b - b'0') as i32)));
        } else {
            if let Some(num) = num.take() {
                nums.push(num);
            }
            match b {
                b'.' => (),
                b'\n' => {
                    pt.1 += 1;
                    pt.0 = -1;
                }
                _ => {
                    syms.insert(pt, b as char);
                }
            }
        }
        pt.0 += 1;
    }

    Schm { nums, syms }
}

fn main() {
    let input = include_str!("../input");
    dbg!(part1(input));
    dbg!(part2(input));
}

fn dcnt(mut n: i32) -> i32 {
    if n == 0 {
        return 1;
    }
    let mut cnt = 0;
    while n > 0 {
        n /= 10;
        cnt += 1;
    }
    cnt
}

fn bbox(pt: Pt, n: i32) -> (std::ops::Range<i32>, std::ops::Range<i32>) {
    ((pt.0 - 1)..(pt.0 + dcnt(n) + 1), (pt.1 - 1)..(pt.1 + 2))
}

fn part1(input: &str) -> i32 {
    let mut sum = 0;
    let schm = parse(input);
    'num: for (pt, n) in &schm.nums {
        let bbox = bbox(*pt, *n);
        for y in bbox.1.clone() {
            for x in bbox.0.clone() {
                if schm.syms.contains_key(&Pt(x, y)) {
                    // dbg!((n, pt, x, y));
                    sum += n;
                    continue 'num;
                }
            }
        }
    }
    sum
}

fn part2(input: &str) -> i32 {
    let schm = parse(input);
    let mut result = 0;
    for star in schm.syms.iter().filter(|e| *e.1 == '*') {
        let (prd, cnt) = schm
            .nums
            .iter()
            .filter(|num| {
                let bbox = bbox(num.0, num.1);
                bbox.0.contains(&star.0 .0) && bbox.1.contains(&star.0 .1)
            })
            .fold((1, 0), |(prd, cnt), ele| (prd * ele.1, cnt + 1));
        if cnt == 2 {
            result += prd;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(4361, part1(input));
        assert_eq!(467835, part2(input));
    }
}
