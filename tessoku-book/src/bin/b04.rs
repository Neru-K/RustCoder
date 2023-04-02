use proconio::input;

fn main() {
    input! {
        mut n: String,
    }

    let mut result = 0;
    let mut x = 1;

    for c in n.chars().rev() {
        if c == '1' {
            result += x;
        }
        x = x * 2;
    }

    println!("{}", result);
}
