use proconio::input;

fn main() {
    input! {
        d: usize,
        n:usize,
        lr:[[usize;2];n],
    }

    let mut b: Vec<isize> = vec![0; d + 1];

    for i in 0..n {
        b[lr[i][0] - 1] += 1;
        b[lr[i][1]] -= 1;
    }

    for j in 1..d {
        b[j] += b[j - 1];
    }

    for k in 0..b.len() - 1 {
        println!("{}", b[k]);
    }
}
