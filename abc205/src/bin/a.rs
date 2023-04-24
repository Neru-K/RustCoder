use proconio::input;

fn main() {
    input! {
        a: f32, b:f32,
    }

    let result = b / 100.0 * a;

    println!("{}", result);
}
