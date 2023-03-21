use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut count = 0;

    for c in s.chars() {
        if c == 'v' {
            count += 1;
        } else if c == 'w' {
            count += 2;
        }
    }

    println!("{}", count);
}
