use proconio::input;

fn main() {
    input! {
        n: usize, q: usize,
        a: [usize;n],
        lr: [[usize;2];q],
    }

    let mut sum = vec![0];

    for i in 1..=n {
        sum.push(sum[i - 1] + a[i - 1]);
    }

    for j in 0..q {
        println!("{}", sum[lr[j][1]] - sum[lr[j][0] - 1]);
    }
}
