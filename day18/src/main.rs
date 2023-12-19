use std::collections::{BTreeSet, BinaryHeap, HashSet, VecDeque};

fn main() {
    let input = include_str!("../input");
    dbg!(part1(input));
    dbg!(part2(input));
}

type Pt = [i32; 2];

#[derive(Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn step(&self, mut pt: Pt) -> Pt {
        match self {
            Dir::Up => pt[1] -= 1,
            Dir::Down => pt[1] += 1,
            Dir::Left => pt[0] -= 1,
            Dir::Right => pt[0] += 1,
        }
        pt
    }
}

fn parse_part1(input: &str) -> Vec<(Dir, i32)> {
    let mut result = vec![];
    for line in input.lines() {
        let mut split = line.split_whitespace();
        let cmd = split.next().unwrap();
        let amt = split.next().unwrap().parse::<i32>().unwrap();
        let _code = split.next().unwrap();

        result.push(match cmd {
            "L" => (Dir::Left, amt),
            "R" => (Dir::Right, amt),
            "U" => (Dir::Up, amt),
            "D" => (Dir::Down, amt),
            _ => unimplemented!(),
        });
    }
    result
}

fn part1(input: &str) -> usize {
    let insts = parse_part1(input);
    area_flood(&insts)
}

fn area_flood(insts: &[(Dir, i32)]) -> usize {
    let mut map = HashSet::new();
    let mut pos = [0, 0];

    for (dir, amt) in insts {
        for _n in 0..*amt {
            match dir {
                Dir::Left => pos[0] -= 1,
                Dir::Right => pos[0] += 1,
                Dir::Up => pos[1] -= 1,
                Dir::Down => pos[1] += 1,
            }
            map.insert(pos);
        }
    }

    // find extremeties of the map
    let mut mins = [0, 0];
    let mut maxs = [0, 0];
    for pos in map.iter() {
        mins[0] = mins[0].min(pos[0]);
        mins[1] = mins[1].min(pos[1]);
        maxs[0] = maxs[0].max(pos[0]);
        maxs[1] = maxs[1].max(pos[1]);
    }
    let xbounds = mins[0] - 1..=maxs[0] + 1;
    let ybounds = mins[1] - 1..=maxs[1] + 1;

    let orig_size = map.len();

    // now flood fill to get outer area
    let mut work = VecDeque::new();
    work.push_back([mins[0] - 1, mins[1] - 1]);
    while let Some(pos) = work.pop_front() {
        if map.contains(&pos) {
            continue;
        }
        if !xbounds.contains(&pos[0]) || !ybounds.contains(&pos[1]) {
            continue;
        }
        map.insert(pos);
        use Dir::*;
        for dir in [Up, Down, Left, Right] {
            work.push_back(dir.step(pos));
        }
    }

    let outside_area = map.len() - orig_size;
    xbounds.count() * ybounds.count() - outside_area
}

#[derive(PartialEq, Eq)]
struct YFirst([i32; 2]);
impl Ord for YFirst {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0[1]
            .cmp(&other.0[1])
            .then(self.0[0].cmp(&other.0[0]))
            .reverse()
    }
}
impl PartialOrd for YFirst {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_part2(input: &str) -> Vec<(Dir, i32)> {
    let mut result = vec![];
    for line in input.lines() {
        let mut split = line.split_whitespace();
        let _cmd = split.next().unwrap();
        let _amt = split.next().unwrap().parse::<i32>().unwrap();
        let code = split
            .next()
            .unwrap()
            .strip_prefix("(#")
            .unwrap()
            .strip_suffix(")")
            .unwrap();
        let (amt, cmd) = code.split_at(5);
        let amt = i32::from_str_radix(amt, 16).unwrap();

        result.push(match cmd {
            "0" | "R" => (Dir::Right, amt),
            "1" | "D" => (Dir::Down, amt),
            "2" | "L" => (Dir::Left, amt),
            "3" | "U" => (Dir::Up, amt),
            _ => unimplemented!(),
        });
    }
    result
}

fn part2(input: &str) -> i64 {
    let insts = parse_part2(input);
    area_smart(&insts)
}

fn area_smart(insts: &[(Dir, i32)]) -> i64 {
    let mut map = BinaryHeap::new();

    let mut pos = [0, 0];

    for (dir, amt) in insts {
        let start_pos = pos;
        match dir {
            Dir::Right => pos[0] += amt,
            Dir::Down => pos[1] += amt,
            Dir::Left => pos[0] -= amt,
            Dir::Up => pos[1] -= amt,
        }
        //println!("{:?} from {:?} to {:?}", dir, start_pos, pos);
        match dir {
            Dir::Up | Dir::Down => {
                map.push(YFirst(start_pos));
                map.push(YFirst(pos));
            }
            _ => (),
        }
    }

    let mut edges = vec![];
    let mut last_y = -1;
    let mut area = 0;
    while let Some(YFirst([_, new_y])) = map.peek() {
        let delta_y = new_y - last_y - 1;
        if delta_y != 0 {
            let delta_area = delta_y as i64
                * edges
                    .chunks(2)
                    .map(|pair| (pair[1] - pair[0] + 1) as i64)
                    .sum::<i64>();
            area += delta_area;
        }
        last_y = *new_y;

        let mut next_edges: BTreeSet<_> = edges.iter().copied().collect();
        while let Some(YFirst([x, y])) = map.peek() {
            if *y != last_y {
                break;
            }
            if next_edges.contains(x) {
                next_edges.remove(x);
            } else {
                next_edges.insert(*x);
            }
            map.pop(); // discard
        }
        assert_eq!(next_edges.len() % 2, 0); // check accounting
        let next_edges: Vec<_> = next_edges.into_iter().collect(); // convert to vec

        // now look for the cross-section of this row
        let mut prev = edges.iter().peekable();
        let mut next = next_edges.iter().peekable();
        let mut start_x = None;
        let mut state = (false, false);
        loop {
            let x = match (prev.peek(), next.peek()) {
                (None, None) => break,
                (None, Some(_n)) => {
                    state.1 = !state.1;
                    next.next().unwrap()
                }
                (Some(_p), None) => {
                    state.0 = !state.0;
                    prev.next().unwrap()
                }
                (Some(p), Some(n)) => {
                    if p < n {
                        state.0 = !state.0;
                        prev.next().unwrap()
                    } else {
                        state.1 = !state.1;
                        next.next().unwrap()
                    }
                }
            };
            if !state.0 && !state.1 {
                let delta_area = (*x - start_x.unwrap() + 1) as i64;
                area += delta_area;
                start_x = None;
            } else if let None = start_x {
                start_x = Some(x);
            }
        }
        assert!(!state.0);
        assert!(!state.1);

        edges = next_edges;
    }

    area
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)";
        assert_eq!(part2(input), 952408144115);
        assert_eq!(area_smart(&parse_part1(input)), 62);
    }
}
