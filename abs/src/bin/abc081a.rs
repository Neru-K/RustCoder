use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut count = 0;

    for c in s.chars() {
        if c == '1' {
            count = count + 1;
        }
    }
    println!("{}", count);
}
