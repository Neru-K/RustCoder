use proconio::input;

fn main() {
    input! {
        a: usize,b:usize,
    }

    let mut result = "No";

    for i in a..=b {
        if 100 % i == 0 {
            result = "Yes";
            break;
        }
    }

    println!("{}", result);
}
