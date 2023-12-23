use std::collections::{HashMap, VecDeque, HashSet};

fn main() {
    let input = include_str!("../input");
    dbg!(part1(input));
    dbg!(part2(input));
}

fn part1(input: &str) -> usize {
    let map = parse(input);
    let mut totals = [0,0];
    let mut state = make_states(&map);
    for _n in 0..1000 {
        let history = broadcast(&map, &mut state);
        for (_,_,highlow) in history {
            totals[highlow as usize] += 1;
        }
    }
    totals[0] * totals[1]
}

fn part2(input: &str) -> usize {
    let map = parse(input);
    let mut state = make_states(&map);
    let mut n: usize = 1;

    let mut watches: HashMap<(&str, &str, u8), (usize, Option<usize>)> = HashMap::new();

    // // find all dependencies of rx
    // fn deps<'a>(map: &'a HashMap<String, Node>, node: &str, out: &mut HashSet<&'a str>) {
    //     if node == "broadcaster" { return; }
    //     for dep in map[node].ins.iter() {
    //         if out.insert(dep.as_str()) {
    //             deps(map, dep, out);
    //         }
    //     }
    // }

    let mut rx_deps = HashSet::new();
    let rx_dep = map.iter().find(|(name, node)| node.outs.iter().any(|n| n == "rx")).unwrap().0;
    for dep in map[rx_dep].ins.iter() {
        rx_deps.insert(dep.as_str());
    }

    'press: loop {
        let history = broadcast(&map, &mut state);
        for signal in history {
            // let (from, to, level) = &signal;
            match watches.get_mut(&signal) {
                None => { watches.insert(signal, (n, None)); },
                Some((last, interval @ None)) => { 
                    let diff = n - *last;
                    *interval = Some(diff);
                    *last = n;
                },
                Some((last, Some(interval))) => {
                    let diff = n - *last;
                    if diff != *interval {
                        *interval = diff;
                    }
                    *last = n;
                },                
            }
        }
        n += 1;

        let mut product = 1;
        for dep in rx_deps.iter() {
            match watches.get(&(*dep, rx_dep, 1)) {
                Some((_last, Some(interval))) => {
                    product *= interval;
                },
                _ => continue 'press,
            }
        }
        return product;
    }
}

fn parse(input: &str) -> HashMap<String, Node> {
    let mut map: HashMap<String, Node> = input
        .lines()
        .map(|line| {
            let (src, dsts) = line.trim().split_once(" -> ").unwrap();
            let (sigil, name) = {
                let (ch0, rest) = src.split_at(1);
                if ch0 == "&" || ch0 == "%" {
                    (ch0.chars().next(), rest)
                } else {
                    (None, src)
                }
            };

            let outs = dsts.split(", ").map(str::to_string).collect();

            let node = Node {
                ins: vec![],
                outs,
                sigil,
            };
            (name.to_string(), node)
        })
        .collect();

    let mut ins: HashMap<String, Vec<String>> = HashMap::new();

    for (src, node) in map.iter_mut() {
        for dst in &node.outs {
            ins.entry(dst.to_string())
                .or_default()
                .push(src.to_string());
        }
    }

    for (name, ins) in ins.drain() {
        map.get_mut(&name).map(|node| node.ins = ins);
    }

    map
}

#[derive(Debug)]
enum NodeState<'a> {
    Conj { ins: HashMap<&'a str, bool> },
    FlipFlop { on: bool },
    Broadcast,
}

struct Node {
    ins: Vec<String>,
    outs: Vec<String>,
    sigil: Option<char>,
}

fn make_states(map: &HashMap<String, Node>) -> HashMap<&str, NodeState> {
    let mut result = HashMap::new();
    for (name, node) in map {
        let state = match node.sigil {
            Some('&') => NodeState::Conj {
                ins: node.ins.iter().map(|n| (n.as_str(), false)).collect(),
            },
            Some('%') => NodeState::FlipFlop { on: false },
            None => NodeState::Broadcast,
            _ => unimplemented!(),
        };
        result.insert(name.as_str(), state);
    }
    result
}

fn broadcast<'a>(
    map: &'a HashMap<String, Node>,
    states: &'_ mut HashMap<&'a str, NodeState<'a>>,
) -> Vec<(&'a str, &'a str, u8)> {
    let mut history = vec![];

    let mut work = VecDeque::new();
    work.push_back(("button", "broadcaster", 0u8));
    while let Some(signal) = work.pop_front() {
        history.push(signal);
        let (src, dest, highlow) = signal;

        let out = match states.get_mut(dest) {
            None => None,
            Some(NodeState::Conj { ins }) => {
                *ins.get_mut(src).unwrap() = highlow != 0;
                if ins.iter().all(|s| *s.1) {
                    Some(0)
                } else {
                    Some(1)
                }
            }
            Some(NodeState::FlipFlop { on }) => {
                if highlow == 1 {
                    None
                } else {
                    *on = !*on;
                    Some(if *on { 1 } else { 0 })
                }
            }
            Some(NodeState::Broadcast) => Some(highlow),
        };

        if let Some(out) = out {
            for next in &map[dest].outs {
                work.push_back((dest, next, out));
            }
        }
    }

    history
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1a() {
        let input = "broadcaster -> a, b, c
        %a -> b
        %b -> c
        %c -> inv
        &inv -> a";
        assert_eq!(part1(input), 32000000);
    }

    #[test]
    fn test_part1b() {
        let input = "broadcaster -> a
        %a -> inv, con
        &inv -> b
        %b -> con
        &con -> output";
        assert_eq!(part1(input), 11687500);
    }
}
