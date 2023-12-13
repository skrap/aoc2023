use std::collections::HashMap;

fn main() {
    let input = include_str!("../input");
    dbg!(part1(input));
    dbg!(part2(input));
}

fn part1(input: &str) -> usize {
    let puzs = parse(input);
    puzs.iter()
        .map(|puz| count_ways(puz.map.as_bytes(), &puz.reqs, &mut HashMap::new()))
        .sum()
}

fn part2(input: &str) -> usize {
    let mut puzs = parse(input);
    for puz in puzs.iter_mut() {
        puz.map = [puz.map.as_str(); 5].join("?");
        puz.reqs = puz.reqs.repeat(5);
    }
    puzs.iter()
        .map(|puz| count_ways(puz.map.as_bytes(), &puz.reqs, &mut HashMap::new()))
        .sum()
}

fn trim_dots(mut map: &[u8]) -> &[u8] {
    while let Some(b'.') = map.get(0) {
        map = &map[1..];
    }
    map
}

fn consume_req(mut map: &[u8], mut req: usize) -> Option<&[u8]> {
    while req > 0 {
        if let Some(b'#' | b'?') = map.get(0) {
            map = &map[1..];
            req -= 1;
        } else {
            return None;
        }
    }
    match map.get(0) {
        Some(b'.') => Some(map),
        None => Some(map),
        Some(b'?') => {
            // must be a dot, so skip it
            Some(&map[1..])
        }
        Some(b'#') => None,
        _ => unimplemented!(),
    }
}

fn count_ways<'a>(
    map: &'a [u8],
    reqs: &'a [usize],
    memo: &mut HashMap<(&'a [u8], &'a [usize]), usize>,
) -> usize {
    let map = trim_dots(map);

    if let Some(result) = memo.get(&(map, reqs)) {
        return *result;
    }

    let result = if let Some(req) = reqs.get(0) {
        // there's a requirement to fulfill
        match map.get(0) {
            Some(b'#') => {
                // consume the requirement
                if let Some(next) = consume_req(map, *req) {
                    count_ways(next, &reqs[1..], memo)
                } else {
                    0
                }
            }
            Some(b'?') => {
                // maybe consume the requirement
                let maybe_yes = if let Some(next) = consume_req(map, *req) {
                    count_ways(next, &reqs[1..], memo)
                } else {
                    0
                };
                let maybe_no = count_ways(&map[1..], reqs, memo);
                maybe_yes + maybe_no
            }
            Some(_) => unimplemented!(),
            None => 0,
        }
    } else {
        // could be ? left, but we better not have any #
        if map.iter().any(|c| *c == b'#') {
            0
        } else {
            1
        }
    };

    memo.insert((map, reqs), result);
    result
}

struct Puzzle {
    map: String,
    reqs: Vec<usize>,
}

fn parse(input: &str) -> Vec<Puzzle> {
    input
        .lines()
        .map(|line| {
            let (map, rest) = line.trim().split_once(" ").unwrap();
            let reqs = rest
                .split(",")
                .map(|n| n.parse::<usize>().unwrap())
                .collect();
            let map = map.to_string();
            Puzzle { map, reqs }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1";
        assert_eq!(21, part1(input));
    }

    #[test]
    fn test_part1_hard() {
        let input = "?###???????? 3,2,1";
        assert_eq!(10, part1(input));
    }

    #[test]
    fn test_part2() {
        let input = "???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1";
        assert_eq!(525152, part2(input));
    }
}
