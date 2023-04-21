use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    let mut result = "No";

    if a != 2 && b != 2 {
        result = "Yes";
    }

    println!("{}", result);
}
