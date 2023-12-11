use std::collections::HashSet;

fn main() {
    let input = include_str!("../input");
    dbg!(part1(input));
    dbg!(part2(input));
}

fn part1(input: &str) -> i64 {
    let map = parse_dilate(input, 1);

    let mut total_dist = 0_i64;
    for galaxy in &map {
        for other in &map {
            total_dist += (galaxy[0]-other[0]).abs() + (galaxy[1]-other[1]).abs();
        }
    }
    // this counts [A,B] as well as [B,A] so divide by 2
    total_dist/2
}

fn part2(input: &str) -> i64 {
    let map = parse_dilate(input, 1000000-1);

    let mut total_dist = 0_i64;
    for galaxy in &map {
        for other in &map {
            total_dist += (galaxy[0]-other[0]).abs() + (galaxy[1]-other[1]).abs();
        }
    }
    // this counts [A,B] as well as [B,A] so divide by 2
    total_dist/2
}

fn parse_dilate(input: &str, dilate_amt: i64) -> HashSet<[i64; 2]> {
    let mut xcounts = vec![0; input.lines().next().unwrap().len()];
    let mut ycounts = vec![0; input.lines().count()];

    let mut result = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.trim().char_indices() {
            if ch == '#' {
                result.insert([x as i64, y as i64]);
                xcounts[x] += 1;
                ycounts[y] += 1;
            }
        }
    }

    // find dilated space
    let xoffset: Vec<i64> = {
        let mut accum = 0;
        xcounts
            .iter()
            .map(|cnt| {
                if *cnt == 0 {
                    accum += dilate_amt;
                }
                accum
            })
            .collect()
    };

    let yoffset: Vec<i64> = {
        let mut accum = 0;
        ycounts
            .iter()
            .map(|cnt| {
                if *cnt == 0 {
                    accum += dilate_amt;
                }
                accum
            })
            .collect()
    };

    let result = result.iter().map(|[x,y]| {
        // add dilation
        [*x + xoffset[*x as usize], *y + yoffset[*y as usize]]
    }).collect();

    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....";
        assert_eq!(part1(input), 374);
    }
}