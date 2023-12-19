use std::collections::{BinaryHeap, HashMap};

fn main() {
    let input = include_str!("../input");
    dbg!(part1(input));
    dbg!(part2(input));
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Dir {
    North,
    South,
    East,
    West,
}

use Dir::*;

impl Dir {
    fn step(&self, pos: [i32; 2]) -> [i32; 2] {
        match self {
            West => [pos[0] - 1, pos[1]],
            East => [pos[0] + 1, pos[1]],
            North => [pos[0], pos[1] - 1],
            South => [pos[0], pos[1] + 1],
        }
    }
    fn left(&self) -> Self {
        match self {
            North => East,
            South => West,
            East => South,
            West => North,
        }
    }
    fn right(&self) -> Self {
        match self {
            North => West,
            South => East,
            East => North,
            West => South,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct State {
    pos: [i32; 2],
    last_dir: Dir,
    straight_steps: u8,
}

#[derive(Eq, PartialEq, Hash)]
struct Item {
    state: State,
    cost: i32,
}

impl Item {
    fn est_cost(&self) -> i32 {
        self.cost - self.state.pos[0] - self.state.pos[1]
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.est_cost().cmp(&other.est_cost()).reverse()
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn part1(input: &str) -> i32 {
    run_cart(input, 0, 3)
}

fn part2(input: &str) -> i32 {
    run_cart(input, 4, 10)
}

fn run_cart(input: &str, min_straight: u8, max_straight: u8) -> i32 {
    // A* algorithm woooooo
    
    let map: Vec<_> = input
        .lines()
        .map(|line| line.trim().bytes().map(|ch| ch - b'0').collect::<Vec<_>>())
        .collect();

    let in_map = |pos: [i32; 2]| {
        (0..map.len() as i32).contains(&pos[1]) && (0..map[0].len() as i32).contains(&pos[0])
    };

    let mut work = BinaryHeap::new();
    let start = Item {
        cost: 0,
        state: State {
            pos: [0, 0],
            last_dir: South,
            straight_steps: 0,
        },
    };
    work.push(start);

    let target = [map[0].len() as i32 - 1, map.len() as i32 - 1];
    let mut best = HashMap::new();
    while let Some(Item { cost, state }) = work.pop() {
        if state.pos == target {
            return cost;
        }
        if let Some(prev) = best.get(&state) {
            if *prev <= cost {
                continue;
            }
        }
        best.insert(state.clone(), cost);

        // turns first
        if state.straight_steps >= min_straight {
            for dir in [state.last_dir.left(), state.last_dir.right()] {
                let pos = dir.step(state.pos);
                if !in_map(pos) {
                    continue;
                }
                let next = Item {
                    cost: cost + map[pos[1] as usize][pos[0] as usize] as i32,
                    state: State {
                        pos,
                        last_dir: dir,
                        straight_steps: 1,
                    },
                };
                work.push(next);
            }
        }

        // straight
        if state.straight_steps < max_straight {
            let pos = state.last_dir.step(state.pos);
            if !in_map(pos) {
                continue;
            }
            let next = Item {
                cost: cost + map[pos[1] as usize][pos[0] as usize] as i32,
                state: State {
                    pos,
                    last_dir: state.last_dir,
                    straight_steps: state.straight_steps + 1,
                },
            };
            work.push(next);
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "2413432311323
        3215453535623
        3255245654254
        3446585845452
        4546657867536
        1438598798454
        4457876987766
        3637877979653
        4654967986887
        4564679986453
        1224686865563
        2546548887735
        4322674655533";
        assert_eq!(102, part1(input));
    }
}
