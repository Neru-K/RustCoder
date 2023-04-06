use proconio::input;

fn main() {
    input! {
        n: usize,
        a:[usize;n],
        q:usize,
        lr:[[usize;2];q],
    }

    let mut sum = vec![0];

    for i in 1..=n {
        sum.push(sum[i - 1] + a[i - 1]);
    }
    //println!("{:?}", sum);

    for j in 0..q {
        let all = lr[j][1] as f32 - lr[j][0] as f32 + 1.0;
        let outcome = sum[lr[j][1]] as f32 - sum[lr[j][0] - 1] as f32;

        if all / 2.0 == outcome {
            println!("draw");
        } else if all / 2.0 < outcome {
            println!("win");
        } else if all / 2.0 > outcome {
            println!("lose");
        }
    }
}
