use proconio::input;

fn main() {
    input! {
        n: i32,
    }

    let mut result = 1;
    let mut max_count = 0;

    for i in 1..=n {
        let mut count = 0;
        let mut num = i;

        while num % 2 == 0 {
            count += 1;
            num = num / 2;
        }
        if count > max_count {
            max_count = count;
            result = i;
        }
    }

    println!("{}", result);
}
