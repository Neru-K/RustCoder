use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let c = vec![
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
        "s", "t", "u", "v", "w", "x", "y", "z",
    ];

    println!("{}", c[n - 97]);
}
