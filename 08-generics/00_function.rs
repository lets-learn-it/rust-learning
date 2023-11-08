
// types which implement Mul & Copy traits
fn square1<T: std::ops::Mul<Output = T> + Copy>(x: T) -> T {
  x * x
}

// using where
fn square2<T>(x: T) -> T
where T: std::ops::Mul<Output = T> + Copy {
  x * x
}

fn main() {
  println!("Square of int: {}", square1(5));
  println!("Square of float: {}", square1(5.5));

  println!("Square of int: {}", square2(5));
  println!("Square of float: {}", square2(5.5));
}