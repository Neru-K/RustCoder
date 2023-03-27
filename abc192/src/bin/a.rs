use proconio::input;

fn main() {
    input! {
        n: i32,
    }

    let md = n % 100;

    println!("{}", 100 - md);
}
