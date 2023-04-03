use proconio::input;

fn main() {
    input! {
        n: usize,k:usize,
    }

    let mut result: usize = 0;

    'outer: for x in 1..=n {
        for y in 1..=n {
            if x + y > k {
                continue 'outer;
            }
            let z = k - x - y;

            if z >= 1 && z <= n {
                result += 1;
            }
        }
    }

    println!("{}", result);
}
