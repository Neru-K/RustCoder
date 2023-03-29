use proconio::input;

fn main() {
    input! {
        n:usize,
        mut a: [usize; n],
    }
    let mut has_even = true;
    let mut count = 0;

    while has_even == true {
        for i in 0..n {
            if a[i] % 2 != 0 {
                has_even = false;
            } else {
                a[i] = a[i] / 2;
            }
        }
        if has_even == true {
            count = count + 1;
        }
    }

    println!("{}", count);
}
