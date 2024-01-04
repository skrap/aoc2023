fn main() {
    let input = include_str!("../input");
    dbg!(part1(input, 200000000000000, 400000000000000));
    dbg!(part2(input));
}

fn part1(input: &str, min: i64, max: i64) -> usize {
    let stones = parse(input);

    let mut result = 0;

    for (aidx, a) in stones.iter().enumerate() {
        for (_bidx, b) in stones.iter().enumerate().skip(aidx + 1) {
            let [dxa, dya] = [a.vel[0] as f64, a.vel[1] as f64];
            let [dxb, dyb] = [b.vel[0] as f64, b.vel[1] as f64];

            let tb = (b.pos[1] - a.pos[1]) as f64 - ((b.pos[0] - a.pos[0]) as f64 * dya / dxa);
            let tb = tb / ((dxb * dya / dxa) - dyb);

            let ta = (b.pos[0] - a.pos[0]) as f64 / dxa + tb * dxb / dxa;

            if tb < 0. || ta < 0. {
                // in the past
                continue;
            }

            let to_check = [
                (a.pos[0], ta * dxa),
                (a.pos[1], ta * dya),
                (b.pos[0], tb * dxb),
                (b.pos[1], tb * dyb),
            ];

            if to_check.into_iter().all(|(base, delta)| {
                ((min - base) as f64) <= delta && ((max - base) as f64) >= delta
            }) {
                result += 1;
            }
        }
    }

    result
}

// Significant hints needed for this one.  key insight is that
// system can be reduced to linear equations with 6 unknowns,
// one for each dimension of the rock's position and velocity.
// To sum up:
// Each hailstone's (pN, vN) intersection with the rock (p, v)
// can be found at time t via pN + t*vN = p * tv.  t is unknown
// but that's OK, because we can rearrange that equation as
// (pN - p) = t * (v - vN).  This means that the two vectors
// (pN - p) and (v - vN) are parallel, as they just differ by
// a constant factor.  Parallel vector have cross product of
// zero.
// Thus (pN - p) x (v - vN) = 0.
// For a given hailstone, e.g. hailstone #3, p3 and v3 are given.
// Thus, this is 3 equations with 3 unknowns.  HOWEVER it isn't
// trivially a linear equation without more work, as the cross-product
// has a p x v in it.
// Second important insight is that this equation can be rearranged
// so that p x v is on the LHS, and this side of the equation is
// independent of the hailstone chosen.
// p x v = p3 x v + p x v3 - p3 x v3
//
// So a different hailstone,  say hailstone #7, can be chosen,
// and set equal to the RHS of the hailstone #3 equation.
// p7 x v + p x v7 - p7 x v7 = p3 x v + p x v3 - p3 x v3
//
// Rearranging to put the unknowns on the LHS and the knowns on the RHS:
// (p7 - p3) x v + p x (v7 - v3) = p7 x v7 - p3 x v3
//
// This new *linear* system has 3 equations and 6 unknowns.  So, we choose another
// pair (checking that the chosen vectors are independent, ie their cross
// products are zero) say hailstone #3 and hailstone #8.
// Thus 6 equations, and 6 unknowns.  Then we just use gaussian elimination
// and we should be able to solve it!

type IVec3 = [i64; 3];

fn sub(a: IVec3, b: IVec3) -> IVec3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn cross(a: IVec3, b: IVec3) -> IVec3 {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn part2(input: &str) -> i64 {
    // find 3 indpendent hailstones.
    let stones = parse(input);

    let mut chosen = vec![stones[0]];
    for s in stones {
        if chosen.iter().all(|ch| {
            // test cross != [0,0,0]
            cross(ch.vel, s.vel) != [0, 0, 0]
        }) {
            chosen.push(s);
            if chosen.len() == 3 {
                break;
            }
        }
    }

    assert_eq!(chosen.len(), 3);

    let s = chosen;

    fn three_eqns(s0: Stone, s1: Stone) -> [IVec3;3] {
        let p01 = sub(s0.pos, s1.pos);
        let v01 = sub(s0.vel, s1.vel);
        let cnst01 = sub(cross(s0.pos, s0.vel), cross(s1.pos, s1.vel));
        [p01, v01, cnst01]
    }

    let [p01, v01, cnst01] = three_eqns(s[0], s[1]);
    let [p02, v02, cnst02] = three_eqns(s[0], s[2]);

    let mut mat = [[0f64;7];6];

    fn fill_mx(m: &mut [[f64;7]], px: IVec3, vx: IVec3, cx: IVec3) {
        for r in 0..3 {
            let i_a = (r+1)%3;
            let i_b = (r+2)%3;

            m[r][3+i_b] = px[i_a] as f64;
            m[r][3+i_a] = -px[i_b] as f64;
            m[r][0+i_a] = vx[i_b] as f64;
            m[r][0+i_b] = -vx[i_a] as f64;
            m[r][6] = cx[r] as f64;
        }
    }

    fill_mx(&mut mat[0..3], p01, v01, cnst01);
    fill_mx(&mut mat[3..6], p02, v02, cnst02);

    // now solve the system via gaussian elimination.  algorithm taken from wikipedia.
    
    gauss_elim(&mut mat);
    solve(&mut mat);
    
    // answer must be integral, so just round everything.
    for ele in mat.iter_mut().map(|r| r.iter_mut()).flatten() {
        *ele = ele.round();
    }
    (mat[0][6] + mat[1][6] + mat[2][6]).round() as i64
}

fn solve(mat: &mut [[f64; 7]; 6]) {
    let m = mat.len();
    let n = mat[0].len();
    
    let mut h = m-1; // pivot row
    let mut k = n-2; // pivot col

    loop {
        let val = mat[h][n-1] / mat[h][k];
        mat[h][n-1] = val;
        mat[h][k] = 1.0;

        for r in 0..h {
            mat[r][n-1] -= mat[r][k] * val;
            mat[r][k] = 0.0; 
        }

        if h == 0 || k == 0 {
            assert_eq!(h,k);  // what have we done?
            break;
        } else {
            h -= 1;
            k -= 1;
        }
    }
}

fn gauss_elim(mat: &mut [[f64; 7]; 6]) {
    let mut h = 0; // pivot row
    let mut k = 0; // pivot col

    let m = mat.len();
    let n = mat[0].len();

    while h < m && k < n {
        /* Find the k-th pivot: */
        let i_max = (h..m).max_by(|i,j| mat[*i][k].partial_cmp(&mat[*j][k]).unwrap()).unwrap();
        // i_max := argmax (i = h ... m, abs(A[i, k]))
        if mat[i_max][k] == 0.0 {
            /* No pivot in this column, pass to next column */
            k = k + 1;
        } else {
            //     swap rows(h, i_max)
            if h < i_max {
                let (r0, r1) = mat.split_at_mut(i_max);
                std::mem::swap(&mut r0[h], &mut r1[0]);
            }
        
            /* Do for all rows below pivot: */
            for i in h+1..m {
                let f = mat[i][k] / mat[h][k];
                /* Fill with zeros the lower part of pivot column: */
                mat[i][k] = 0.0;
                /* Do for all remaining elements in current row: */
                for j in k+1..n {
                    mat[i][j] = mat[i][j] - mat[h][j] * f;
                }
            }
            h = h + 1;
            k = k + 1;
        }
    }
}

#[derive(Debug,Clone, Copy)]
struct Stone {
    pos: [i64; 3],
    vel: [i64; 3],
}

fn parse(input: &str) -> Vec<Stone> {
    input
        .lines()
        .map(|line| {
            // 280761666456810, 405119910828575, 63496246448680 @ 6, -304, 412
            let nums: Vec<i64> = line
                .trim()
                .split(" @ ")
                .flat_map(|s| s.split(", "))
                .filter_map(|s| s.parse::<i64>().ok())
                .collect();
            Stone {
                pos: nums[0..3].try_into().unwrap(),
                vel: nums[3..6].try_into().unwrap(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "19, 13, 30 @ -2, 1, -2
        18, 19, 22 @ -1, -1, -2
        20, 25, 34 @ -2, -2, -4
        12, 31, 28 @ -1, -2, -1
        20, 19, 15 @ 1, -5, -3";
        assert_eq!(part1(input, 7, 27), 2);
    }
}
