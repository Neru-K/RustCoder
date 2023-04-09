use proconio::input;

fn main() {
    input! {
        t: usize, //closing time
        n: usize, //number of emplyees
        lr:[(usize,usize);n],
    }

    let mut nums: Vec<usize> = vec![0; t + 1];

    for i in 0..n {
        nums[lr[i].0] += 1;
        nums[lr[i].1 + 1] -= 1;
    }

    println!("{:?}", nums);

    for j in 1..nums.len() {
        nums[j] += nums[j - 1];
    }
    println!("{:?}", nums);

    for k in 1..nums.len() - 1 {
        println!("{}", nums[k]);
    }
}
