use proconio::input;

fn main() {
    input! {
        n: i32,
    }

    let result = countOddDigit(n);
    println!("{}", result);
}

fn countOddDigit(n: i32) -> i32 {
    let mut count = 0;
    for i in 1..=n {
        if i >= 1 && i <= 9 {
            count += 1;
        } else if i >= 100 && i <= 999 {
            count += 1;
        } else if i >= 10000 && i <= 99999 {
            count += 1;
        }
    }
    count
}
