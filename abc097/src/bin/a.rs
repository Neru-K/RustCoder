use proconio::input;

fn main() {
    input! {
        mut abc: [isize;3],d:isize
    }

    let mut result = "No";

    if (abc[1] - abc[0]) * (abc[1] - abc[0]) <= d * d
        && (abc[2] - abc[1]) * (abc[2] - abc[1]) <= d * d
    {
        result = "Yes";
    } else if (abc[2] - abc[0]) * (abc[2] - abc[0]) <= d * d {
        result = "Yes";
    }

    println!("{}", result);
}
