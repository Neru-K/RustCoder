use proconio::input;

fn main() {
    input! {
        n: usize,x:i32,
        mut a:[i32;n],
    }

    let mut result = "No";

    'outer: for i in 0..n {
        for j in 0..n {
            if a[i] - a[j] == x {
                result = "Yes";
                break 'outer;
            }
        }
    }

    println!("{}", result);
}
