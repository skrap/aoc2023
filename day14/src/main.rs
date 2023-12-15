use std::collections::HashMap;

fn main() {
    let input = include_str!("../input");
    dbg!(part1(input));
    dbg!(part2(input, 1000000000));
}

#[derive(Clone, Debug, Eq, PartialEq, Copy, Hash)]
enum Tile {
    Cube,
    Round,
    Space,
}

fn part1(input: &str) -> usize {
    let map = parse(input);

    let next = roll_map(map);

    // print_map(&next);

    score(next)
}

fn part2(input: &str, spin_cnt: usize) -> usize {
    let map = parse(input);

    let mut prev_states = HashMap::new();
    
    let mut next = map;
    let mut skipped_rolls = 0;
    for n in 1.. {
        next = spin_map(roll_map(next));
        if n + skipped_rolls == 4 * spin_cnt {
            return score(next);
        }

        if let Some(prev_n) = prev_states.get(&next) {
            // print_map(&next);
            let loop_len = n - prev_n;
            let remaining = 4 * spin_cnt - n;
            skipped_rolls = (remaining / loop_len) * loop_len;
        }
        prev_states.insert(next.clone(), n);
    }

    unimplemented!()
}

fn roll_map(map: Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    use Tile::*;
    let mut next: Vec<Vec<Tile>> = vec![];
    for (rowidx, row) in map.iter().enumerate() {
        let mut nextrow = vec![];
        for (colidx, tile) in row.iter().enumerate() {
            match tile {
                Round => {
                    // rock n roll
                    let mut idx = rowidx;
                    while idx > 0 && next[idx-1][colidx] == Space {
                        idx -= 1;
                    }
                    if idx == rowidx {
                        nextrow.push(Round);
                    } else {
                        nextrow.push(Space);
                        next[idx][colidx] = Round;
                    }
                },
                Cube => nextrow.push(Cube),
                Space => nextrow.push(Space),
            }
        }
        next.push(nextrow);
    }
    next
}

fn spin_map(map: Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    let mut next = vec![vec![Tile::Space;map.len()]; map.len()];
    for mapy in 0..map.len() {
        for mapx in 0..map[0].len() {
            next[mapx][map.len()-mapy-1] = map[mapy][mapx];
        }
    }
    next
}

fn parse(input: &str) -> Vec<Vec<Tile>> {
    use Tile::*;
    let map: Vec<Vec<Tile>> = input.lines().map(|line| {
        line.chars().map(|ch| match ch {
            '#' => Cube,
            'O' => Round,
            '.' => Space,
            _ => unimplemented!()
        }).collect()
    }).collect();

    assert_eq!(map.len(), map[0].len());

    map
}

#[allow(unused)]
fn print_map(next: &Vec<Vec<Tile>>) {
    use Tile::*;
    for row in next.iter() {
        for tile in row {
            print!("{}", match tile {
                Cube => '#',
                Round => 'O',
                Space => '.',
            });
        }
        println!();
    }
}

fn score(next: Vec<Vec<Tile>>) -> usize {
    use Tile::*;
    // score the map
    let height = next.len();
    let mut score = 0;
    for (rowidx, row) in next.iter().enumerate() {
        let rounds = row.iter().filter(|t| **t == Round).count();
        score += rounds * (height - rowidx);
    }
    score
}
