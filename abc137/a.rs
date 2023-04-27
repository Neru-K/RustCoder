use proconio::input;

fn main() {
  input! {
      a: isize,b:isize,
  }
  
   let mut max = a + b;
  
  if a - b > max {
  	max = a - b;
  }
  if a * b > max {
  	max = a * b;
  }
  
  println!("{}",max);
}