use proconio::input;

fn main() {
    input! {
        n: usize,
        a:[usize;n],
    }
    let mut result = "No";

    'outer: for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            for k in 0..n {
                if i == k || j == k {
                    continue;
                }

                if a[i] + a[j] + a[k] == 1000 {
                    result = "Yes";
                    break 'outer;
                }
            }
        }
    }

    println!("{}", result);
}
