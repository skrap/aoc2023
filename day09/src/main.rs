fn main() {
    let input = include_str!("../input");
    dbg!(part1(input));
    dbg!(part2(input));
}

fn part1(input: &str) -> i64 {
    input.lines().map(extrap).sum()
}

fn part2(input: &str) -> i64 {
    input.lines().map(extrap_back).sum()
}

fn extrap(line: &str) -> i64 {
    let levels = levels(line);
    let mut result = 0;
    for n in levels.iter().rev() {
        result += n.last().unwrap();
    }
    result
}

fn extrap_back(line: &str) -> i64 {
    let levels = levels(line);
    let mut result = 0;
    for n in levels.iter().rev() {
        result = n[0] - result;
    }
    result
}

fn levels(line: &str) -> Vec<Vec<i64>> {
    let mut levels: Vec<Vec<i64>> = vec![];
    levels.push(
        line.split_whitespace()
            .map(|n| n.parse::<i64>().unwrap())
            .collect(),
    );
    loop {
        let next: Vec<_> = levels
            .last()
            .unwrap()
            .windows(2)
            .map(|pair| pair[1] - pair[0])
            .collect();
        let done = next.iter().all(|n| *n == 0);
        if done { break; }
        levels.push(next);
    }
    levels
}
