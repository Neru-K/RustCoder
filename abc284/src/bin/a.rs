use proconio::input;

fn main() {
    input! {
        n:usize,
        mut s: [String; n],
    }

    s.reverse();

    let result = s
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join("\n");

    println!("{}", result);
}
