fn main() {
    let input = include_str!("../input");
    dbg!(part1(input));
    dbg!(part2(input));
}

fn part1(input: &str) -> usize {
    input.split("\n\n").map(|input| score(input, 0)).sum()
}

fn part2(input: &str) -> usize {
    input.split("\n\n").map(|input| score(input, 1)).sum()
}

fn score(input: &str, smudges: u32) -> usize {
    let (rows, cols) = parse(input);

    for (nums, scale) in [(&rows, 100), (&cols, 1)] {
        for start in 1..nums.len() {
            let mut second = start;
            let mut first = second - 1;
            let mut diffs = 0;
            let found = loop {
                diffs += (nums[first] ^ nums[second]).count_ones();
                if diffs > smudges {
                    break false;
                }
                if first == 0 {
                    break diffs == smudges;
                }
                first -= 1;
                second += 1;
                if second == nums.len() {
                    break diffs == smudges;
                }
            };
            if found {
                return scale * start;
            }
        }
    }
    unimplemented!()
}

fn parse(input: &str) -> (Vec<usize>, Vec<usize>) {
    let lines: Vec<_> = input.lines().map(str::as_bytes).collect();
    let rows: Vec<_> = lines
        .iter()
        .map(|&line| {
            line.iter().fold(0usize, |acc, n| {
                (acc << 1) | (if *n == b'#' { 1 } else { 0 })
            })
        })
        .collect();

    let cols: Vec<_> = (0..lines[0].len())
        .map(|col| {
            (0..lines.len()).fold(0usize, |acc, row| {
                (acc << 1) | if lines[row][col] == b'#' { 1 } else { 0 }
            })
        })
        .collect();
    (rows, cols)
}
