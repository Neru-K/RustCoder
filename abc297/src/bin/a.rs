use proconio::input;

fn main() {
    input! {
        n: usize, d:usize,
        t: [usize;n],
    }
    let mut result: isize = -1;

    for i in 1..n {
        if t[i] - t[i - 1] <= d {
            result = t[i] as isize;
            break;
        }
    }
    println!("{}", result);
}
