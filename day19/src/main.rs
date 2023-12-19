struct Part([i32;4]);
impl Part {
    fn rating(&self) -> i32 {
        self.0.iter().sum()
    }
    fn idx(ele: &str) -> usize {
        match ele {
            "x" => 0, 
            "m" => 1, 
            "a" => 2, 
            "s" => 3, 
            _ => unimplemented!(),
        }
    }
    fn get(&self, ele: &str) -> i32 {
        let idx = Part::idx(ele);
        self.0[idx]
    }
}

struct Flow {
    conds: Vec<(String,String,String,String)>,
    default: String,
}

impl Flow {
    fn eval<'a> (&'a self, part: &Part) -> Eval<'a> {
        for (left, cmp, right, next) in self.conds.iter() {
            let left = left.parse().unwrap_or_else(|_| part.get(left));
            let right = right.parse().unwrap_or_else(|_| part.get(right));
            let result = match cmp.as_str() {
                "<" => left < right,
                ">" => left > right,
                _ => todo!(),
            };
            if result {
                return Eval::new(next);
            }
        }
        Eval::new(&self.default)
    }
}

#[derive(Debug, Clone, Copy)]
struct PartRange([[i32;2];4]);
impl PartRange {
    fn cnt_valid(&self) -> i64 {
        self.0.iter().map(|[min,max]| (max-min+1) as i64).product()
    }

    fn at_most(&mut self, idx: usize, n: i32) {
        self.0[idx][1] = self.0[idx][1].min(n)
    }

    fn at_least(&mut self, idx: usize, n: i32) {
        self.0[idx][0] = self.0[idx][0].max(n)
    }
}

enum Eval<'a> {
    Accept,
    Reject,
    NextFlow(&'a str)
}
impl<'a> Eval<'a> {
    fn new(name: &'a str) -> Self {
        match name {
            "A" => Accept,
            "R" => Reject,
            other => NextFlow(other),
        }
    }
}

use std::collections::{HashMap, VecDeque};

use Eval::*;

fn main() {
    let input = include_str!("../input");
    dbg!(part1(input));
    dbg!(part2(input));
}

fn part1(input: &str) -> i32 {
    let (flows, parts) = parse(input);

    let mut result = 0;
    for part in &parts {
        let mut flow = "in";
        let rating = loop {
            match flows[flow].eval(part) {
                Accept => break part.rating(),
                Reject => break 0,
                NextFlow(next) => flow = next,
            }
        };
        result += rating;
    }
    result
}

fn part2(input: &str) -> i64 {
    let (flows, _parts) = parse(input);

    let range = PartRange([[1,4000];4]);

    let mut work = VecDeque::new();
    work.push_back((range, "in"));
    let mut accepted = 0;
    while let Some((mut range, flowname)) = work.pop_front() {
        if flowname == "A" {
            accepted += range.cnt_valid();
            continue;
        }
        if flowname == "R" || range.cnt_valid() == 0 {
            continue;
        }
        let flow = &flows[flowname];
        for (var, cmp, num, next) in &flow.conds {
            // normalize to have the variable on the left
            let mut cmp = cmp.as_str();
            if var.as_bytes()[0].is_ascii_digit() {
                std::mem::swap(&mut &var, &mut &num);
                cmp = match cmp {
                    "<" => ">",
                    ">" => "<",
                    _ => unimplemented!()
                }
            }

            let idx = Part::idx(var);
            let num = num.parse().unwrap();
            
            match cmp {
                "<" => { 
                    // var < num
                    let mut new = range;
                    new.at_most(idx, num-1);
                    work.push_back((new, next));

                    // var >= num
                    range.at_least(idx, num);
                },
                ">" => {
                    // var > num
                    let mut new = range;
                    new.at_least(idx, num+1);
                    work.push_back((new, next));

                    // var <= num
                    range.at_most(idx, num);
                },
                _ => unimplemented!(),
            }
        }

        work.push_back((range, flow.default.as_str()));
    }
    accepted
}

fn parse(input: &str) -> (HashMap<String, Flow>, Vec<Part>) {
    let mut flows = HashMap::new();
    let mut parts = vec![];

    let (rules_txt, parts_txt) = input.split_once("\n\n").unwrap();

    for line in rules_txt.lines() {
        let (name,rest) = line.split_once("{").unwrap();
        let rest = rest.strip_suffix("}").unwrap();

        let mut conds = vec![];

        for rule in rest.split(",") {
            let mut it = rule.split_inclusive(['<','>',':']);
            match (it.next(),it.next(),it.next()) {
                (Some(left_op), Some(right_colon), Some(next)) => {
                    let (left,op) = left_op.split_at(left_op.len()-1);
                    let (right, _) = right_colon.split_at(right_colon.len()-1);
                    conds.push((left.to_string(),op.to_string(),right.to_string(),next.to_string()))
                }
                (Some(default), None, None) => {
                    flows.insert(name.to_string(), Flow {
                        conds,
                        default: default.to_string(),
                    });
                    break;
                }
                _ => unreachable!()
            }
        }
    }

    for part in parts_txt.lines() {
        let mut nums = part.split(|ch: char| !ch.is_numeric()).filter_map(|s| s.parse::<i32>().ok());
        parts.push(Part([
            nums.next().unwrap(),
            nums.next().unwrap(),
            nums.next().unwrap(),
            nums.next().unwrap(),
        ]))
    }

    (flows, parts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = include_str!("../test_input");
        assert_eq!(part2(input), 167409079868000);
    }
}