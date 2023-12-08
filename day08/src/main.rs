use std::collections::HashMap;

fn main() {
    let input = include_str!("../input");
    dbg!(part1(input));
    dbg!(part2(input));
}

fn part1(input: &str) -> usize {
    let (dirs, map) = parse(input);
    let mut pos = "AAA";
    for (idx, ch) in dirs.trim().chars().cycle().enumerate() {
        if pos == "ZZZ" {
            return idx;
        }
        pos = match ch {
            'L' => map[pos].0,
            'R' => map[pos].1,
            _ => unimplemented!(),
        };
    }
    unreachable!()
}

fn part2(input: &str) -> usize {
    let (dirs, map) = parse(input);

    let mut loops: Vec<_> = map
        .keys()
        .filter(|k| k.ends_with("A"))
        .map(|start| {
            let mut pos = *start;
            let mut ends = HashMap::new();
            for (idx, ch) in dirs.trim().chars().cycle().enumerate() {
                if pos.ends_with("Z") {
                    if let Some(prev_idx) = ends.get(pos) {
                        // found loop
                        assert_eq!(ends.len(), 1); // if there are loops with multiple end states, let's hear about it.
                        assert_eq!(idx - prev_idx, *prev_idx); // is each finish and the end of the loop?
                        return idx - prev_idx;
                    }
                    // println!("{}: idx {} found {}", start, idx, pos);
                    ends.insert(pos, idx);
                }
                pos = match ch {
                    'L' => map[pos].0,
                    'R' => map[pos].1,
                    _ => unimplemented!(),
                };
            }
            unreachable!();
        })
        .collect();
    // println!("{:?}", loops);

    // now just need to find the LCM of all loops
    // let's implement it ourselves for funsies, from
    // https://en.wikipedia.org/wiki/Binary_GCD_algorithm

    fn gcd(a: usize, b: usize) -> usize {
        match (a, b) {
            (0, n) => n,
            (n, 0) => n,
            (n, m) if (n & 1) == 0 && (m & 1) == 0 => gcd(n >> 1, m >> 1) << 1,
            (n, m) if (n & 1) == 0 => gcd(n >> 1, m),
            (n, m) if (m & 1) == 0 => gcd(n, m >> 1),
            (n, m) if n >= m => gcd(n - m, m),
            (n, m) => gcd(m - n, n),
        }
    }
    fn lcm(a: usize, b: usize) -> usize {
        a * (b / gcd(a,b))
    }
    let lcm_all = loops.iter().fold(loops[0], |n,m| lcm(n,*m));
    lcm_all
}

fn parse(input: &str) -> (&str, HashMap<&str, (&str, &str)>) {
    let mut result = HashMap::new();
    let (dirs, rest) = input.split_once("\n\n").unwrap();
    for line in rest.lines() {
        // RGT = (HDG, QJV)
        result.insert(&line[0..3], (&line[7..10], &line[12..15]));
    }
    (dirs, result)
}
