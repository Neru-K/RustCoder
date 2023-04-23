use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        mut v: [usize;5],
    }

    let set: HashSet<_> = v.drain(..).collect();
    v.extend(set.into_iter());

    println!("{}", v.len());
}
