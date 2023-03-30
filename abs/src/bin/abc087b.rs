use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        x: usize,
    }

    let mut count = 0;

    for i in 0..=a {
        for j in 0..=b {
            for k in 0..=c {
                let sum = 500 * i + 100 * j + 50 * k;
                if sum == x {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}
