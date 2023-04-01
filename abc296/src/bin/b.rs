use proconio::input;

fn main() {
    input! {
        s: [String;8],
    }

    let alphabet = ["a", "b", "c", "d", "e", "f", "g", "h"];

    for row in 0..8 {
        for (i, c) in s[row].chars().enumerate() {
            if c == '*' {
                println!("{}{}", alphabet[i], 8 - row);
            }
        }
    }
}
