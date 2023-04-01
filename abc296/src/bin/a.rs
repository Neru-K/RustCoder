use proconio::input;

fn main() {
    input! {
        _n: usize,
        s: String,
    }

    let mut result = "Yes";
    let mut prev = None;

    for c in s.chars() {
        if let Some(p) = prev {
            if c == p {
                result = "No";
                break;
            }
        }
        prev = Some(c);
    }
    println!("{}", result);
}
