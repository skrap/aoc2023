use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input");
    dbg!(part1(input));
    dbg!(part2(input));
}

const COL_DIM: usize = 10;

#[derive(Debug, Clone)]
struct Brick {
    shadow: [[bool; COL_DIM]; COL_DIM],
    height: i16,
    base_elevation: i16,
    supported_by: HashSet<usize>,
}

impl Brick {
    fn from(line: &str) -> Self {
        let line = line.trim();
        // 2,0,5~2,2,5
        let coords: Vec<i16> = line
            .split([',', '~'])
            .filter_map(|x| x.parse::<i16>().ok())
            .collect();
        let [ax, ay, az, bx, by, bz] = coords.try_into().unwrap();
        assert!((0..COL_DIM as i16).contains(&ax));
        assert!((0..COL_DIM as i16).contains(&ay));
        assert!((0..COL_DIM as i16).contains(&bx));
        assert!((0..COL_DIM as i16).contains(&by));
        let height = az.max(bz) - az.min(bz) + 1;
        let mut shadow: [[bool; COL_DIM]; COL_DIM] = Default::default();
        let mut it = [ax, ay];
        shadow[it[0] as usize][it[1] as usize] = true;
        while it != [bx, by] {
            it[0] += (bx-ax).signum();
            it[1] += (by-ay).signum();
            shadow[it[0] as usize][it[1] as usize] = true;
        }
        Self {
            shadow,
            height,
            base_elevation: az.min(bz),
            supported_by: Default::default(),
        }
    }
}

fn parse(input: &str) -> Vec<Brick> {
    let mut bricks: Vec<_> = input.lines().map(|line| Brick::from(line)).collect();
    bricks.sort_by_key(|b| b.base_elevation);
    bricks
}

fn part1(input: &str) -> usize {
    let (fallen, _) = fall(&parse(input));

    let cricital_bricks: HashSet<_> = fallen.iter().filter_map(|brick| {
        if brick.supported_by.len() == 1 {
            Some(brick.supported_by.iter().next().unwrap())
        } else {
            None
        }
    }).collect();
    fallen.len() - cricital_bricks.len()
}

fn part2(input: &str) -> usize {
    let (fallen, _) = fall(&parse(input));
    let mut result = 0;
    for n in 0..fallen.len() {
        let mut test = fallen.clone();
        test.remove(n);
        result += fall(&test).1;
    }
    result
}

fn fall(bricks: &[Brick]) -> (Vec<Brick>, usize) {
    let mut bricks = bricks.to_vec();
    bricks.sort_by_key(|b| b.base_elevation);

    #[derive(Debug, Clone, Copy)]
    struct Elev {
        height: i16,
        brick_id: Option<usize>,
    }

    let mut elevations = [[Elev {
        height: 0,
        brick_id: None,
    }; COL_DIM]; COL_DIM];

    let mut fall_cnt = 0;
    
    for (brick_id, brick) in bricks.iter_mut().enumerate() {
        brick.supported_by.clear();
        // find new elevation
        let mut elevation = 0i16;
        for y in 0..COL_DIM {
            for x in 0..COL_DIM {
                let ele = &elevations[x][y];
                if brick.shadow[x][y] && ele.height >= elevation {
                    elevation = ele.height;
                }
            }
        }
        if brick.base_elevation != elevation {
            fall_cnt += 1;
        }
        brick.base_elevation = elevation;

        // find all supporting bricks
        for y in 0..COL_DIM {
            for x in 0..COL_DIM {
                if brick.shadow[x][y] {
                    let ele = &mut elevations[x][y];
                    if ele.height == elevation {
                        if let Some(id) = ele.brick_id {
                            brick.supported_by.insert(id);
                        }
                    }
                    ele.brick_id = Some(brick_id);
                    ele.height = elevation + brick.height;
                }
            }
        }
    }
    (bricks, fall_cnt)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "1,0,1~1,2,1
        0,0,2~2,0,2
        0,2,3~2,2,3
        0,0,4~0,2,4
        2,0,5~2,2,5
        0,1,6~2,1,6
        1,1,8~1,1,9";
        assert_eq!(part1(input), 5);
    }
}