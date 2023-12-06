fn main() {
    let input = include_str!("../input");
    dbg!(part1(input));
    dbg!(part2(input));
}

fn part1(input: &str) -> usize {
    let (seeds, maps) = parse(input);
    seeds
        .iter()
        .map(|n| {
            let mut n = *n;
            for m in maps.iter() {
                for tx in m.txs.iter() {
                    if tx.contains(n) {
                        n = tx.map(n);
                        break;
                    }
                }
                // println!("{}: {}", m.name, n);
            }
            n
        })
        .min()
        .unwrap()
}

fn part2(input: &str) -> usize {
    let (seeds, maps) = parse(input);
    let seeds: Vec<Rng> = seeds
        .chunks(2)
        .map(|chnk| Rng {
            min: chnk[0],
            max: chnk[0] + chnk[1] - 1,
        })
        .collect();

    let mut result = vec![];
    for seed in seeds {
        let mut rngs = vec![seed];
        for map in &maps {
            let next = rngs
                .drain(..)
                .flat_map(|rng| map.map_range(rng).into_iter())
                .collect();
            rngs = next;
        }
        result.extend(rngs);
    }
    result.iter().min().unwrap().min
}

fn parse(input: &str) -> (Vec<usize>, Vec<Map>) {
    let (seeds, rest) = input.split_once("\n\n").unwrap();
    let seeds = seeds
        .split_whitespace()
        .filter_map(|n| n.parse::<usize>().ok())
        .collect();
    let mut maps = vec![];
    for mapstr in rest.split("\n\n") {
        let (name, rest) = mapstr.split_once("\n").unwrap();
        let mut txs: Vec<Tx> = rest.lines().map(to_tx).collect();
        txs.sort();
        maps.push(Map {
            name: name.to_string(),
            txs,
        });
    }
    (seeds, maps)
}

struct Map {
    #[allow(unused)]
    name: String,
    txs: Vec<Tx>,
}

impl Map {
    /// takes one seed range, e.g. [10, 5] and produces the ranges it maps
    /// to through all transitions in the map, e.g. [2, 3], [16, 2]
    fn map_range(&self, mut seed: Rng) -> Vec<Rng> {
        #[allow(unused)]
        let orig_seed = seed;
        let mut result = vec![];
        for tx in &self.txs {
            if seed.len() == 0 {
                break;
            }
            if seed.min < tx.from {
                // [seed    [tx...
                if seed.max < tx.from {
                    // no overlap, we're done.
                    break;
                }
                let new_seed = Rng {
                    min: seed.min,
                    max: tx.from - 1,
                };
                result.push(new_seed);
                seed.min = tx.from;
            }
            if seed.len() == 0 {
                break;
            }

            // seed must start >= [tx
            if seed.min < tx.from + tx.len {
                // [seed...  tx], so map the portion within the tx
                let overlap_max = seed.max.min(tx.from + tx.len - 1);
                let new_seed = Rng {
                    min: tx.map(seed.min),
                    max: tx.map(overlap_max),
                };
                result.push(new_seed);
                seed.min = overlap_max + 1;
            }
        }
        if seed.len() > 0 {
            result.push(seed);
        }
        // println!("{:?}: {:?}", orig_seed, result);
        result
    }

}

#[derive(PartialEq, PartialOrd, Ord, Eq)]
struct Tx {
    from: usize,
    to: usize,
    len: usize,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
struct Rng {
    min: usize,
    max: usize,
}

impl Rng {
    fn len(&self) -> usize {
        if self.max < self.min {
            0
        } else {
            self.max + 1 - self.min
        }
    }
}

impl Tx {
    fn contains(&self, from: usize) -> bool {
        let range = self.from..self.from + self.len;
        range.contains(&from)
    }

    fn map(&self, n: usize) -> usize {
        assert!(self.contains(n));
        self.to + (n - self.from)
    }
}

fn to_tx(input: &str) -> Tx {
    let mut ns = input
        .split_whitespace()
        .filter_map(|n| n.parse::<usize>().ok());
    Tx {
        to: ns.next().unwrap(),
        from: ns.next().unwrap(),
        len: ns.next().unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(35, part1(include_str!("../part1test")));
    }

    #[test]
    fn test_part2() {
        assert_eq!(46, part2(include_str!("../part1test")));
    }
}
