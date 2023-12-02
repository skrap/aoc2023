#[derive(Default)]
struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

impl Cubes {
    fn max(&self, other: &Cubes) -> Cubes {
        Self {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }
}

struct Game {
    id: usize,
    cubes: Vec<Cubes>,
}

fn main() {
    let input = include_str!("../input");
    dbg!(part1(input));
    dbg!(part2(input));
}

fn parse(input: &str) -> Vec<Game> {
    let mut result = vec![];
    for line in input.lines() {
        let (id, rest) = line.split_once(": ").unwrap();
        let id = id.split_once(" ").unwrap().1.parse().unwrap();
        let mut cubes = vec![];
        for shown in rest.split("; ") {
            let (mut red, mut blue, mut green) = (0, 0, 0);
            for (n, color) in shown.split(", ").map(|pair| pair.split_once(" ").unwrap()) {
                let n: u32 = n.parse().unwrap();
                match color {
                    "red" => red += n,
                    "green" => green += n,
                    "blue" => blue += n,
                    _ => unimplemented!(),
                }
            }
            cubes.push(Cubes { red, green, blue });
        }
        result.push(Game { id, cubes });
    }
    result
}

fn part1(input: &str) -> usize {
    let mut result = 0;
    let gs = parse(input);
    for game in &gs {
        // max cubes shown:
        let maxes = game
            .cubes
            .iter()
            .fold(Cubes::default(), |acc, ele| acc.max(ele));
        if maxes.red > 12 || maxes.green > 13 || maxes.blue > 14 {
            // impossible game
        } else {
            result += game.id;
        }
    }
    result
}

fn part2(input: &str) -> u32 {
    let mut result = 0;
    let gs = parse(input);
    for game in &gs {
        // max cubes shown:
        let maxes = game
            .cubes
            .iter()
            .fold(Cubes::default(), |acc, ele| acc.max(ele));
        
        let power = maxes.red * maxes.green * maxes.blue;
        result += power;
        
    }
    result
}
