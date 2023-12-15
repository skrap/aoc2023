use std::collections::VecDeque;

fn main() {
    let input = include_str!("../input");
    dbg!(part1(input));
    dbg!(part2(input));
}

fn part1(input: &str) -> usize {
    input.trim().split(",").map(hash).sum::<usize>()
}

fn hash(input: &str) -> usize {
    let mut acc = 0u8;
    for ch in input.as_bytes() {
        acc = acc.wrapping_add(*ch).wrapping_mul(17);
    }
    acc as usize
}

fn part2(input: &str) -> usize {
    
    #[derive(Debug,Clone)]
    struct LensLabel<'a> {
        label: &'a str,
        focal: &'a str,
    }
    let mut boxes: Vec<VecDeque<LensLabel>> = vec![VecDeque::new(); 256];

    for instr in input.trim().split(",") {
        let (label, op, focal) = {
            let mut parser = instr.split_inclusive(&['-', '=']);
            let label_op = parser.next().unwrap();
            let (label, op) = label_op.split_at(label_op.len()-1);
            (label, op, parser.next())
        };
        let boxnum = hash(label);
        match op {
            "=" => {
                let boxn = &mut boxes[boxnum];
                let found = boxn.iter_mut().any(|lens| {
                    if lens.label == label {
                        lens.focal = focal.unwrap();
                        true
                    } else {
                        false
                    }
                });
                if !found {
                    boxn.push_back(LensLabel {
                        label,
                        focal: focal.unwrap(),
                    });
                }
            },
            "-" => {
                let boxn = &mut boxes[boxnum];
                if let Some(n) = boxn.iter().position(|lens| lens.label == label) {
                    boxn.drain(n..=n);
                }
            },
            _ => unimplemented!(),
        }
    }

    // score it
    let mut score = 0;
    for boxn in 0..boxes.len() {
        for (idx, lens) in boxes[boxn].iter().enumerate() {
            score += (boxn+1)*(idx+1)*lens.focal.parse::<usize>().unwrap(); 
        }
    }
    score
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(part2(input), 145);
    }
}