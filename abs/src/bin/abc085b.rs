use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        mut d:[usize;n],
    }

    d.sort();
    let set: HashSet<_> = d.drain(..).collect();
    d.extend(set.into_iter());
    println!("{}", d.len());
}
