use std::collections::{HashMap, HashSet, VecDeque};
use Dir::*;

fn main() {
    let input = include_str!("../input");
    dbg!(part1(input));
    dbg!(part2(input, 26501365_usize));
}

type Pos = [i64; 2];

#[derive(Debug, Clone, Copy)]
enum Dir {
    North,
    South,
    East,
    West,
}

impl Dir {
    fn step(&self, pos: Pos) -> Pos {
        self.stepn(pos, 1)
    }
    fn stepn(&self, pos: Pos, amt: i64) -> Pos {
        match self {
            Dir::North => [pos[0], pos[1] - amt],
            Dir::South => [pos[0], pos[1] + amt],
            Dir::East => [pos[0] + amt, pos[1]],
            Dir::West => [pos[0] - amt, pos[1]],
        }
    }
}

struct Map {
    start: Pos,
    size: Pos,
    rocks: HashSet<Pos>,
}

impl Map {
    fn contains(&self, pos: Pos) -> bool {
        (0..self.size[0]).contains(&pos[0]) && (0..self.size[1]).contains(&pos[1])
    }
}

fn part1(input: &str) -> usize {
    let map = parse(input);
    let mut reachable = HashSet::new();
    reachable.insert(map.start);
    for _step in 0..64 {
        let mut next = HashSet::new();
        for pos in reachable {
            for dir in [North, South, East, West] {
                let newpos = dir.step(pos);
                if map.contains(newpos) && !map.rocks.contains(&newpos) {
                    next.insert(newpos);
                }
            }
        }
        reachable = next;
    }
    reachable.len()
}

fn parse(input: &str) -> Map {
    let mut start = None;
    let mut rocks = HashSet::new();
    let size = [
        input.lines().next().unwrap().trim().len() as i64,
        input.lines().count() as i64,
    ];
    dbg!(size);
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.trim().chars().enumerate() {
            if ch == '#' {
                rocks.insert([x as i64, y as i64]);
            }
            if ch == 'S' {
                start = Some([x as i64, y as i64]);
            }
        }
    }
    Map {
        start: start.unwrap(),
        size,
        rocks,
    }
}

fn part2(input: &str, steps: usize) -> usize {
    let map = parse(input);

    let mut cache = HashMap::new();

    struct CacheEntry {
        reach: HashMap<Pos, usize>,
        max_dist: usize,
        even_reach: usize,
        odd_reach: usize,
    }

    impl CacheEntry {
        fn reachable_cnt(&self, steps: usize) -> usize {
            if steps >= self.max_dist {
                match steps % 2 {
                    0 => self.even_reach,
                    1 => self.odd_reach,
                    _ => unreachable!(),
                }
            } else {
                self.reach
                    .values()
                    .filter(|cost| **cost <= steps && **cost % 2 == steps % 2)
                    .count()
            }
        }

        #[allow(unused)]
        fn print(&self, steps: usize, length: i64) {
            println!("steps: {}, total: {}", steps, self.reachable_cnt(steps));
            for y in 0..length {
                for x in 0..length {
                    match self.reach.get(&[x, y]) {
                        Some(x) if *x <= steps => print!("{}", x%10),
                        Some(_x) => print!("."),
                        None => {
                            print!(" ")
                        }
                    };
                }
                println!();
            }
            println!();
        }
    }

    fn calc_min_reach(map: &Map, start: Pos) -> CacheEntry {
        let mut result = HashMap::new();
        let mut work = VecDeque::new();
        work.push_back((start, 0));
        while let Some((pos, cost)) = work.pop_front() {
            let recurse = match result.get_mut(&pos) {
                Some(min) if cost < *min => {
                    *min = cost;
                    true
                }
                Some(_min) => false, // better min elsewhere - ignore
                None => {
                    result.insert(pos, cost);
                    true
                }
            };
            if recurse {
                for dir in [North, South, East, West] {
                    let newpos = dir.step(pos);
                    if map.contains(newpos) && !map.rocks.contains(&newpos) {
                        work.push_back((newpos, cost + 1));
                    }
                }
            }
        }

        let result = CacheEntry {
            max_dist: *result.values().max().unwrap(),
            even_reach: result.values().filter(|cost| *cost % 2 == 0).count(),
            odd_reach: result.values().filter(|cost| *cost % 2 == 1).count(),
            reach: result,
        };
        // result.print(result.max_dist, map.size[0] as i64);
        result
    }

    assert_eq!(map.start, [map.size[0] / 2, map.size[1] / 2]);
    let tile = map.size[0];

    // 9 situations to account for -
    // 1 tile where we're centered
    cache.insert(map.start, calc_min_reach(&map, map.start));
    // cache[&map.start].print(steps, map.size[0] as i64);

    // start with centered tile. easy.
    let mut result = cache[&map.start].reachable_cnt(steps);
    let tile = tile as i64;

    // cardinals first
    for dir in [North, East, West, South] {
        let mut dir_reachable = 0;
        let mut remain = steps as i64 - (tile/ 2 + 1);
        let tile_start = dir.stepn(map.start, -tile / 2);
        assert_eq!(cache[&map.start].reach[&tile_start], tile as usize / 2);
        let reach = cache
            .entry(tile_start)
            .or_insert_with(|| calc_min_reach(&map, tile_start));
        // reach.print(remain, map.size[0] as i64);

        while remain >= 0 {
            dir_reachable += reach.reachable_cnt(remain as usize);
            remain -= tile;
        }

        result += dir_reachable;
    }

    // diagonals
    for dir in [
        // 4 corners
        [0, 0],
        [tile - 1, 0],
        [tile - 1, tile - 1],
        [0, tile - 1],
    ] {
        let mut dir_reachable = 0;
        let mut remain = steps as i64 - ((map.start[0] + map.start[1] + 2));
        let reach = cache.entry(dir).or_insert_with(|| calc_min_reach(&map, dir));

        let mut tiles = 0;
        while remain >= 0 {
            tiles += 1;
            dir_reachable += tiles * reach.reachable_cnt(remain as usize);
            // reach.print(remain as usize, map.size[0] as i64);
            remain -= tile;
        }

        result += dir_reachable;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brute() {
        let base = include_str!("../input");
        let mut start = String::new();
        for line in base.lines() {
            start += &line.replace("S", ".");
            start.push_str(line);
            start += &line.replace("S", ".");
            start.push('\n');
        }
        let nostart = start.replace("S", ".");
        let input = nostart.clone() + &start + &nostart;

        for n in 65..68 {
            println!("{}", n);
            assert_eq!(part2(base, n), part2(&input, n), "failed at {}", n);
        }
    }

    #[test]
    fn test_brute2() {
        let base
     = "...........
        ......##.#.
        .###..#..#.
        ..#.#...#..
        ....#.#....
        .....S.....
        .##......#.
        .......##..
        .##.#.####.
        .##...#.##.
        ...........";
        let mut start = String::new();
        for line in base.lines() {
            let line = line.trim();
            start += &line.replace("S", ".");
            start += &line.replace("S", ".");
            start.push_str(line);
            start += &line.replace("S", ".");
            start += &line.replace("S", ".");
            start.push('\n');
        }
        let nostart = start.replace("S", ".");
        let input = nostart.clone() + &nostart + &start + &nostart + &nostart;

        println!("{}", input);

        for n in 23..25 {
            println!("{}", n);
            assert_eq!(part2(base, n), part2(&input, n), "failed at {}", n);
        }
    }
}
