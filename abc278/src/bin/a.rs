use proconio::input;

fn main() {
    input! {
        n: usize,
        mut k: usize,
        mut a: [usize; n],
    }

    if k > n {
        k = n;
    }

    a.drain(..k);
    a.extend((0..k).map(|_| 0));
    let s = a
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    println!("{}", s);
}
