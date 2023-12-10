use std::collections::{HashMap, VecDeque};

fn main() {
    let input = include_str!("../input");
    dbg!(part1_and_2(input));
}

fn part1_and_2(input: &str) -> (usize,usize) {
    let map = parse(input);

    // find S
    let start_pos = (0..map.0.len())
        .filter_map(|y| {
            (0..map.0[y].len())
                .filter_map(|x| {
                    if map.0[y][x] == NORTH | EAST | WEST | SOUTH {
                        Some([x as i64, y as i64])
                    } else {
                        None
                    }
                })
                .next()
        })
        .next()
        .unwrap();

    let mut dists = HashMap::new();
    
    let mut work = VecDeque::new();
    work.push_back((0usize,start_pos));
    while let Some((cost, pos)) = work.pop_front() {
        if let Some(prev_cost) = dists.get(&pos) {
            if *prev_cost <= cost {
                continue; // been here cheaper before
            }
        }
        dists.insert(pos, cost);

        let checks = [
            ([pos[0]+0, pos[1]-1], NORTH, SOUTH),
            ([pos[0]+1, pos[1]+0], EAST, WEST),
            ([pos[0]+0, pos[1]+1], SOUTH, NORTH),
            ([pos[0]-1, pos[1]+0], WEST, EAST),
        ];
        let here = map.get(pos).unwrap();

        for (there_pos, here_req, there_req) in checks {
            if here & here_req == 0 {
                continue;
            }
            if let Some(tile) = map.get(there_pos) {
                if tile & there_req != 0 {
                    work.push_back((cost + 1, there_pos));
                }
            }
        }
    }

    let (_,max_dist) = dists.iter().max_by_key(|(_k,v)| **v).unwrap();

    // for part 2, we need to have the start square replaced by its correct tile
    let mut start_tile = 0;
    let checks = [
        ([start_pos[0]+0, start_pos[1]-1], NORTH, SOUTH),
        ([start_pos[0]+1, start_pos[1]+0], EAST, WEST),
        ([start_pos[0]+0, start_pos[1]+1], SOUTH, NORTH),
        ([start_pos[0]-1, start_pos[1]+0], WEST, EAST),
    ];
    for (there, here_req, there_req) in checks {
        if let Some(tile) = map.get(there) {
            if tile & there_req != 0 {
                start_tile |= here_req;
            }
        }
    }
    let mut map = map;
    map.0[start_pos[1] as usize][start_pos[0] as usize] = start_tile;

    let mut inside_cnt = 0;
    
    for y in 0..map.0.len() {
        let mut state = 0;
        for x in 0..map.0[0].len() {
            // cast a ray across a row, and see how many spaces are inside
            let pos = [x as i64, y as i64];
            let is_edge = dists.contains_key(&pos);
            let tile = map.get(pos).unwrap();
            if is_edge {
                state ^= tile & (NORTH|SOUTH);
                // match state {
                //     NORTH => print!("N"),
                //     SOUTH => print!("S"),
                //     n if n == NORTH|SOUTH => print!("*"),
                //     0 => print!("0"),
                //     _=> unimplemented!()
                // }
            } else if state == NORTH|SOUTH {
                inside_cnt += 1;
                // print!("I");
            } else {
                // print!("O");
            }
        }
        println!();
    }

    (*max_dist,inside_cnt)
}

struct Map(Vec<Vec<u8>>);

impl Map {
    fn get(&self, pos: [i64;2]) -> Option<u8> {
        if pos[1] >= 0 {
            if let Some(row) = self.0.get(pos[1] as usize) {
                if pos[0] >= 0 {
                    return row.get(pos[0] as usize).copied();
                } 
            }
        }
        None
    }
}

const NORTH: u8 = 1 << 0;
const EAST: u8 = 1 << 1;
const SOUTH: u8 = 1 << 2;
const WEST: u8 = 1 << 3;

fn parse(input: &str) -> Map {
    Map(input
        .lines()
        .map(|line| {
            line.trim().chars()
                .map(|ch| match ch {
                    '|' => NORTH | SOUTH,
                    '-' => EAST | WEST,
                    'L' => NORTH | EAST,
                    'J' => NORTH | WEST,
                    '7' => WEST | SOUTH,
                    'F' => SOUTH | EAST,
                    '.' => 0,
                    'S' => NORTH | EAST | WEST | SOUTH,
                    _ => unimplemented!()
                })
                .collect()
        })
        .collect())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complex() {
        let input = 
        "FF7FSF7F7F7F7F7F---7
         L|LJ||||||||||||F--J
         FL-7LJLJ||||||LJL-77
         F--JF--7||LJLJ7F7FJ-
         L---JF-JLJ.||-FJLJJ7
         |F|F-JF---7F7-L7L|7|
         |FFJF7L7F-JF7|JL---7
         7-L-JL7||F7|L7F-7F7|
         L.L7LFJ|||||FJL7||LJ
         L7JLJL-JLJLJL--JLJ.L";
        assert_eq!(part1_and_2(input).1, 10);
    }

    #[test]
    fn test_simpler() {
        let input = "...........
        .S-------7.
        .|F-----7|.
        .||.....||.
        .||.....||.
        .|L-7.F-J|.
        .|..|.|..|.
        .L--J.L--J.
        ...........";
        assert_eq!(part1_and_2(input).1, 4);
    }
}