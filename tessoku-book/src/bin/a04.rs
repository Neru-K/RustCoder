use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    for x in (0..10).rev() {
        let wari = 2usize.pow(x);
        print!("{}", (n / wari) % 2);
    }
    println!();
}
