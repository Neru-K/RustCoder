use proconio::input;

fn main() {
    input! {
        n:usize,
        s: [String; n],
    }

    let en = vec!["and", "not", "that", "the", "you"];
    let mut result = "No";

    'outer: for i in 0..s.len() {
        for j in 0..en.len() {
            if s[i] == en[j] {
                result = "Yes";
                break 'outer;
            }
        }
    }
    println!("{}", result);
}
