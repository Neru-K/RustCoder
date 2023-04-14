use proconio::input;

fn main() {
    let mut result = 0;

    input! {
        n: usize,x:usize,
        p: [usize;n],
    }

    for i in 0..n {
        if p[i] == x {
            result = i + 1;
        }
    }

    println!("{}", result)
}
