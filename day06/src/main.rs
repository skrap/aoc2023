struct Race {
    time: usize,
    record: usize,
}

fn parse(input: &str) -> Vec<Race> {
    let mut lines = input.lines();
    let times = lines
        .next()
        .unwrap()
        .split_whitespace()
        .filter_map(|n| n.parse::<usize>().ok());
    let dists = lines
        .next()
        .unwrap()
        .split_whitespace()
        .filter_map(|n| n.parse::<usize>().ok());
    times
        .into_iter()
        .zip(dists.into_iter())
        .map(|(time, record)| Race { time, record })
        .collect()
}

fn main() {
    let input = include_str!("../input");
    dbg!(part1(input));
}

fn part1(input: &str) -> usize {
    let mut product = 1;
    for race in parse(input) {
        let wins = (0..=race.time)
            .filter(|hold| hold * (race.time - hold) > race.record)
            .count();
        product *= wins;
    }
    product
}

