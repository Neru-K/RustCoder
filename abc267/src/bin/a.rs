use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let date = vec!["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];

    for i in 0..date.len() {
        if s == date[i] {
            println!("{}", 5 - i);
            break;
        }
    }
}
