fn main() {
    dbg!(part1(include_str!("../input")));
    dbg!(part2(include_str!("../input")));
}

fn part1(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let mut seen = None;
        for ch in line.chars() {
            if let Some(num) = ch.to_digit(10) {
                if seen.is_none() {
                    sum += num * 10;
                }
                seen = Some(num);
            }
        }
        sum += seen.unwrap();
    }
    sum
}

fn part2(input: &str) -> u32 {
    let mut sum = 0;

    for mut line in input.lines() {
        let mut seen = None;

        while line.len() > 0 {
            let num = if let Some(n) = line.chars().next().and_then(|n| n.to_digit(10)) {
                Some(n)
            } else {
                let nums = [
                    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
                ];
                nums.iter()
                    .enumerate()
                    .filter_map(|(idx, name)| {
                        if line.starts_with(name) {
                            return Some(idx as u32 + 1);
                        }
                        None
                    })
                    .next()
            };

            if let Some(num) = num {
                if seen.is_none() {
                    sum += 10 * num;
                }
                seen = Some(num);
            }
            (_, line) = line.split_at(1);
        }
        sum += seen.unwrap();
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";
        assert_eq!(part2(input), 281);
    }
}
