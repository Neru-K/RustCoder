use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut result = "Won";
    let mut prev_char: Option<char> = None;
    for c in s.chars() {
        if let Some(prev) = prev_char {
            if c != prev {
                result = "Lost";
                break;
            }
        }
        prev_char = Some(c);
    }

    println!("{}", result);
}
