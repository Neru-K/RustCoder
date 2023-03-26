use itertools::Itertools;
use proconio::{fastout, input, marker::Chars}; // 追加

#[fastout]
fn main() {
    input! {
        r:usize,c:usize,
        b: [Chars; r],
    }

    let mut ans: Vec<Vec<char>> = b.clone();

    for i in 0..r {
        for j in 0..c {
            if '1' <= b[i][j] && b[i][j] <= '9' {
                let k: usize = b[i][j] as usize - '0' as usize;
                for i1 in 0..r {
                    for j1 in 0..c {
                        if i1.abs_diff(i) + j1.abs_diff(j) <= k {
                            ans[i1][j1] = '.';
                        }
                    }
                }
            }
        }
        for r in ans {
            println!("{}", r.iter().join(""));
        }
    }
}
