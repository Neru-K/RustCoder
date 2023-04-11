use proconio::input;

fn main() {
    input! {
        t: usize, //closing time
        n: usize, //number of emplyees
        lr:[(usize,usize);n],
    }

    let mut nums: Vec<isize> = vec![0; t + 1];

    for i in 0..n {
        nums[lr[i].0] += 1;
        nums[lr[i].1] -= 1;
    }

    for j in 1..nums.len() {
        nums[j] += nums[j - 1];
    }

    for k in 0..nums.len() - 1 {
        println!("{}", nums[k]);
    }
}
