use proconio::input;

fn main() {
    input! {
        d: usize,
        n:usize,
        lr:[[usize;2];n],
    }

    let mut b: Vec<isize> = vec![0; d];
    //let mut result: Vec<usize> = vec![0; d];

    println!("{:?}", b);

    for i in 0..n {
        println!("lr[i][0]{}", lr[i][0]);
        b[lr[i][0] - 1] += 1;
        println!("lr[i][0]{}", lr[i][1]);
        b[lr[i][1] - 1] -= 1;

        println!("{:?}", b);
    }

    //result[0] = b[0];

    for j in 1..=d {
        println!("{}", b[j - 1] + b[j]);
    }
}
