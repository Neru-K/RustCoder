use proconio::input;

fn main() {
    input! {
        a: usize,b:usize,
    }
    let mut result = b;

    if a <= 5 {
        result = 0;
    } else if a <= 12 {
        result = b / 2;
    }

    println!("{}", result);
}
