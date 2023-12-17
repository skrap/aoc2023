fn main() {
    let input = include_str!("../input");
    dbg!(part1(input));
    dbg!(part2(input));
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

impl Dir {
    fn step(&self, pos: [i64; 2]) -> [i64; 2] {
        match *self {
            Left => [pos[0] - 1, pos[1]],
            Right => [pos[0] + 1, pos[1]],
            Up => [pos[0], pos[1] - 1],
            Down => [pos[0], pos[1] + 1],
        }
    }
}

use std::{collections::HashSet, iter::repeat};
use Dir::*;

fn part1(input: &str) -> usize {
    let map: Vec<Vec<u8>> = input.lines().map(|line| line.trim().bytes().collect()).collect();

    let start_pos = [0, 0];
    let start_dir = Right;
    let mut seen = HashSet::new();
    energize(start_pos, start_dir, &map, &mut seen);
    
    let seen_pos: HashSet<_> = seen.iter().map(|(pos,_dir)| *pos).collect();

    // print_map(&map, &seen);
    seen_pos.len()
}

fn part2(input: &str) -> usize {
    let map: Vec<Vec<u8>> = input.lines().map(|line| line.trim().bytes().collect()).collect();

    let left = repeat(Right).zip(repeat(0i64).zip(0..map.len() as i64));
    let right = repeat(Left).zip(repeat(map[0].len() as i64 - 1).zip(0..map.len() as i64));
    let down = repeat(Down).zip((0..map[0].len() as i64).zip(repeat(0)));
    let up = repeat(Up).zip((0..map.len() as i64).zip(repeat(map.len() as i64-1)));

    left.chain(right).chain(up).chain(down).map(|(dir, pos)|{
        let mut seen = HashSet::new();
        energize([pos.0, pos.1], dir, &map, &mut seen);
        let seen_pos: HashSet<_> = seen.iter().map(|(pos,_dir)| *pos).collect();
        seen_pos.len()
    }).max().unwrap()
}

#[allow(unused)]
fn print_map(map: &Vec<Vec<u8>>, seen: &HashSet<([i64; 2], Dir)>) {
    for (rowidx, row) in map.iter().enumerate() {
        for (colidx, tile) in row.iter().enumerate() {
            if [Up,Down,Left,Right].iter().any(|dir|
                seen.contains(&([colidx as i64, rowidx as i64], *dir))) {
                    print!("#");
            } else {
                print!("{}", char::from(*tile));
            }

        }
        println!();
    }
}

fn energize(pos: [i64; 2], dir: Dir, map: &Vec<Vec<u8>>, seen: &mut HashSet<([i64; 2], Dir)>) {
    if !(0..map.len() as i64 ).contains(&pos[1]) || !(0..map[0].len() as i64).contains(&pos[0]) {
        // off the map - done
        return;
    }
    if seen.insert((pos,dir)) == false {
        // been here before - done
        return;
    }
    // print_map(map, seen);
    // println!();

    match (map[pos[1] as usize][pos[0] as usize], dir) {
        (b'.', _) => energize(dir.step(pos), dir, map, seen),
        // mirror /
        (b'/', Right) => energize(Up.step(pos), Up, map, seen),
        (b'/', Down) => energize(Left.step(pos), Left, map, seen),
        (b'/', Up) => energize(Right.step(pos), Right, map, seen),
        (b'/', Left) => energize(Down.step(pos), Down, map, seen),
        // mirror \
        (b'\\', Right) => energize(Down.step(pos), Down, map, seen),
        (b'\\', Up) => energize(Left.step(pos), Left, map, seen),
        (b'\\', Left) => energize(Up.step(pos), Up, map, seen),
        (b'\\', Down) => energize(Right.step(pos), Right, map, seen),
        // splitter -
        (b'-', Left | Right) => energize(dir.step(pos), dir, map, seen),
        (b'-', Up|Down) => {
            energize(Left.step(pos), Left, map, seen);
            energize(Right.step(pos), Right, map, seen);
        }
        // splitter |
        (b'|', Up | Down) => energize(dir.step(pos), dir, map, seen),
        (b'|', Left | Right) => {
            energize(Up.step(pos), Up, map, seen);
            energize(Down.step(pos), Down, map, seen);
        }
        _ => todo!(),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r".|...\....
        |.-.\.....
        .....|-...
        ........|.
        ..........
        .........\
        ..../.\\..
        .-.-/..|..
        .|....-|.\
        ..//.|....";
        assert_eq!(46, part1(input));    
    }
}