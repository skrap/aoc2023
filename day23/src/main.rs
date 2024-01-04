use std::collections::{HashSet, VecDeque, HashMap, BTreeSet};

fn main() {
    let input = include_str!("../input");
    dbg!(part2_simplify(input, true));
    dbg!(part2_simplify(input, false));
}

#[derive(PartialEq, Eq)]
enum Dir {
    North,
    South,
    East,
    West,
}
use Dir::*;

type Pos = [i64; 2];

impl Dir {
    fn step(&self, pos: Pos) -> Pos {
        match self {
            Dir::North => [pos[0], pos[1] - 1],
            Dir::South => [pos[0], pos[1] + 1],
            Dir::East => [pos[0] + 1, pos[1]],
            Dir::West => [pos[0] - 1, pos[1]],
        }
    }
}

fn part1(input: &str, hills: bool) -> usize {
    let map: Map<'_> = parse(input);

    let start = [1, 0];
    let end = [(map.0[0].len() - 2) as i64, (map.0.len() - 1) as i64];

    let mut work = VecDeque::new();
    work.push_back((start, start, BTreeSet::new(), 0));
    let mut longest = 0;
    let mut test = HashMap::new();
    while let Some((pos, prev, ixs, steps)) = work.pop_front() {
        if pos == end {
            if steps > longest {
                println!("longest: {}", steps);
            }
            longest = longest.max(steps);
        } else {
            let mut choices = vec![];
            for dir in [North, South, East, West] {
                let next = dir.step(pos);
                let legal = match map.get(next) {
                    Some(b'#') => false,
                    None => false,
                    Some(b'.') => true,
                    Some(b'>') if !hills || dir == East => true,
                    Some(b'<') if !hills || dir == West => true,
                    Some(b'^') if !hills || dir == North => true,
                    Some(b'v') if !hills || dir == South => true,
                    _ => false,
                };
                if legal && next != prev && !ixs.contains(&next) {
                    choices.push(next);
                }
            }
            if choices.len() == 1 {
                work.push_back((choices[0], pos, ixs, steps+1));
            }
            else {
                let mut ixs = ixs.clone();
                let entry = test.entry((pos,ixs.clone())).or_insert(0);
                *entry += 1;
                if *entry > 3 {
                    println!("{}", *entry);
                }
                ixs.insert(pos);
                for next in choices {
                    work.push_back((next, pos, ixs.clone(), steps + 1));
                }
            }
        }
    }

    longest
}

fn part2(input: &str, hills: bool) -> usize {
    let map = parse(input);

    #[derive(Debug, PartialEq, Eq, Hash)]
    struct State {
        from: Pos,
        to: Pos,
        prev: Pos,
        ixs_visited: BTreeSet<Pos>
    }

    fn longest(map: &Map, hills: bool, state: State, memo: &mut HashMap<State,Option<usize>>) -> Option<usize> {
        if let Some(val) = memo.get(&state) {
            return *val;
        }
        
        let mut current = state.from;
        let mut prev = state.prev;
        
        let mut steps = 0;

        loop {
            steps += 1;
            let mut choices = vec![];
            for dir in [North, South, East, West] {
                let next = dir.step(current);
                let legal = match map.get(next) {
                    Some(b'#') => false,
                    None => false,
                    Some(b'.') => true,
                    Some(b'>') if !hills || dir == East => true,
                    Some(b'<') if !hills || dir == West => true,
                    Some(b'^') if !hills || dir == North => true,
                    Some(b'v') if !hills || dir == South => true,
                    _ => false,
                };
                if next == state.to {
                    return Some(steps);
                }
                if legal && next != prev && !state.ixs_visited.contains(&next) {
                    choices.push(next);
                }
            }
            match choices.len() {
                0 => {
                    // dead end
                    memo.insert(state, None);
                    return None;
                } 
                1 => {
                    prev = current;
                    current = choices[0];
                    // follow the yellow brick road...
                }
                _ => {
                    println!(r#""{:?}" -- "{:?}" [label:"{}"]"#, state.prev, current, steps);
                    // fork in the road
                    let mut best = None;
                    for choice in choices {
                        let mut ixs_next = state.ixs_visited.clone();
                        ixs_next.insert(current);
                        let next = State {
                            from: choice,
                            to: state.to,
                            prev: current,
                            ixs_visited: ixs_next,
                        };
                        best = best.max(longest(map, hills, next, memo));
                    }
                    let best = best.map(|s| s + steps);
                    memo.insert(state, best);
                    return best;
                }
            }
        }
    }

    let start = State {
        from: [1, 0],
        to: [(map.0[0].len() - 2) as i64, (map.0.len() - 1) as i64],
        prev: [1,0],
        ixs_visited: Default::default(),
    };
    longest(&map, hills, start, &mut HashMap::new()).unwrap()
}

struct Map2 {
    nodes: HashMap<Pos, Vec<(Pos,usize)>>
}

fn part2_simplify(input: &str, hills: bool) -> usize {
    let map: Map<'_> = parse(input);

    fn next_ix(map: &Map, hills: bool, pos: Pos, prev: Pos) -> (Pos,usize) {
        let mut current = pos;
        let mut prev = prev;
        let mut steps = 0;

        loop {
            steps += 1;
            let mut choices = vec![];
            for dir in [North, South, East, West] {
                let next = dir.step(current);
                let legal = match map.get(next) {
                    Some(b'#') => false,
                    None => false,
                    Some(b'.') => true,
                    Some(b'>') if !hills || dir == East => true,
                    Some(b'<') if !hills || dir == West => true,
                    Some(b'^') if !hills || dir == North => true,
                    Some(b'v') if !hills || dir == South => true,
                    _ => false,
                };
                if legal && next != prev {
                    choices.push(next);
                }
            }
            match choices.len() {
                0 => {
                    // dead end
                    return (current, steps);
                } 
                1 => {
                    prev = current;
                    current = choices[0];
                    // follow the yellow brick road...
                }
                _ => {
                    // println!(r#""{:?}" -- "{:?}" [label:"{}"]"#, state.prev, current, steps);
                    // fork in the road
                    return (current, steps);
                }
            }
        }
    }

    let mut work = VecDeque::new();
    let start = [1, 0];
    let end = [(map.0[0].len() - 2) as i64, (map.0.len() - 1) as i64];
    work.push_back(start);
    let mut map2 = HashMap::new();
    while let Some(start_ix) = work.pop_front() {
        if map2.contains_key(&start_ix) {
            continue;
        }

        let mut edges = vec![];
        for dir in [North, South, East, West] {
            let next = dir.step(start_ix);
            let legal = match map.get(next) {
                Some(b'#') => false,
                None => false,
                Some(b'.') => true,
                Some(b'>') if !hills || dir == East => true,
                Some(b'<') if !hills || dir == West => true,
                Some(b'^') if !hills || dir == North => true,
                Some(b'v') if !hills || dir == South => true,
                _ => false,
            };
            if legal {
                let result = next_ix(&map, hills, next,start_ix);
                work.push_back(result.0);
                edges.push(result);
            }
        }
        map2.insert(start_ix, edges);
    }

    assert!(map2.contains_key(&end));
    // now we have a simple map of nodes and edges

    println!("{:?}", &map2);

    let mut work = VecDeque::new();
    work.push_front((start, 0, HashSet::new()));
    let mut longest = 0;
    while let Some((node, steps, mut seen)) = work.pop_back() {
        if node == end {
            longest = longest.max(steps);
            continue;
        }

        seen.insert(node);

        for (next_node, cost) in &map2[&node] {
            if ! seen.contains(next_node) {
                work.push_back((*next_node, steps + cost, seen.clone()));
            }
        }
    }

    longest
}

struct Map<'a>(Vec<&'a [u8]>);

impl Map<'_> {
    fn get(&self, pos: Pos) -> Option<u8> {
        if pos[0] < 0 || pos[1] < 0 {
            return None;
        }
        self.0
            .get(pos[1] as usize)
            .and_then(|row| row.get(pos[0] as usize))
            .and_then(|ch| Some(*ch))
    }
}

fn parse(input: &str) -> Map<'_> {
    Map(input.lines().map(|line| line.trim().as_bytes()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = include_str!("../test");
        assert_eq!(part2(input, false), 154);
    }
}