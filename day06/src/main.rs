struct Race {
    time: usize,
    record: usize,
}

fn parse1(input: &str) -> Vec<Race> {
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
    dbg!(part2(input));
}

fn part1(input: &str) -> usize {
    let mut product = 1;
    for race in parse1(input) {
        let wins = (0..=race.time)
            .filter(|hold| hold * (race.time - hold) > race.record)
            .count();
        product *= wins;
    }
    product
}

fn parse2(input: &str) -> Race {
    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse()
        .unwrap();
    let record = lines
        .next()
        .unwrap()
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse()
        .unwrap();
    Race { time, record }
}

fn part2(input: &str) -> usize {
    // bsearch to find lower boundary between losing and winning
    let race = parse2(input);
    let mut bounds = (0usize,race.time/2);
    println!("Race: {} time {} dist", race.time, race.record);
    while bounds.0 + 1 < bounds.1 {
        let probe = bounds.0 + (bounds.1-bounds.0)/2;
        let dist = probe * (race.time - probe);
        if dist > race.record {
            bounds.1 = probe;
        } else {
            bounds.0 = probe;
        }
        println!("{} is {}, bounds: [{}, {}]", probe, if dist>race.record {"win "} else {"loss"}, bounds.0, bounds.1);
    }
    race.time - 2*bounds.1 + 1
}
