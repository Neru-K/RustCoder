use proconio::input;

fn main() {
    input! {
        n: usize,x:i32,
        a :[i32;n],
    }

    let mut is_exist = "No";

    for i in 0..a.len() {
        if a[i] == x {
            is_exist = "Yes";
        }
    }

    println!("{}", is_exist);
}
