use proconio::input;

fn main() {
    input! {
        n: usize,
        mut num:[usize;n],
    }

    num.sort();
    num.reverse();
    let mut bob = 0;
    let mut alice = 0;

    for i in 0..num.len() {
        if i % 2 == 0 {
            alice += num[i];
        } else {
            bob += num[i];
        }
    }
    println!("{}", alice - bob);
}
