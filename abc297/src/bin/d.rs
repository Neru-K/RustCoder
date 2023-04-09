use proconio::input;

fn main() {
    input! {
        mut a: u64,mut b:u64,
    }

    let mut count = 0;

    while a != b {
        if a > b {
            a = a - b;
        } else {
            b = b - a;
        }
        count += 1;
    }

    println!("{}", count);
}
