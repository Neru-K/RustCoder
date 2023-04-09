use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut fb: isize = -1;
    let mut sb: isize = -1;
    let mut fr: isize = -1;
    let mut sr: isize = -1;
    let mut k: isize = -1;
    let mut result = "No";

    for (i, c) in s.chars().enumerate() {
        if c == 'B' {
            if fb == -1 {
                fb = i as isize + 1;
            } else {
                sb = i as isize + 1;
            }
        }
        if c == 'R' {
            if fr == -1 {
                fr = i as isize + 1;
            } else {
                sr = i as isize + 1;
            }
        }
        if c == 'K' {
            k = i as isize + 1;
        }
    }

    if ((fb % 2 == 1 && sb % 2 == 0) || (fb % 2 == 0 && sb % 2 == 1)) && (fr < k && k < sr) {
        result = "Yes";
    }

    println!("{}", result);
}
