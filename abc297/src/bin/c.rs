use proconio::input;

fn main() {
    input! {
        h: usize,_w:usize,
        mut s:[String;h],
    }

    for row in 0..h {
        while s[row].contains("TT") {
            s[row] = s[row].replace("TT", "PC");
        }
        println!("{}", s[row]);
    }
}
