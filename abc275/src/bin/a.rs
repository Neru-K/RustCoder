use proconio::input;

fn main() {
    input! {
        n: usize,
        mut h:[usize;n]
    }

    let mut highest = 0;

    for i in 0..h.len() {
        if h[i] > h[highest] {
            highest = i;
        }
    }

    println!("{}", highest + 1);
}
