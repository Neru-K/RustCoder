use proconio::input;

fn main() {
    input! {
        k: usize,
    }

    let letters = vec![
        "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R",
        "S", "T", "U", "V", "W", "X", "Y", "Z",
    ];

    for i in 0..k {
        print!("{}", letters[i]);
    }

    println!();
}
