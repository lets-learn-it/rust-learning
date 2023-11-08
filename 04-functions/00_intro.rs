
fn main() {
  greet("Parikshit");

  println!("Sum: {}", sum(4, 5));

  println!("Subtract: {}", subtract(4, 5));

  println!("min max: {:?}", min_max(344, 45));
}

fn greet(name: &str)  {
  println!("Hello, {}", name);
}

fn sum(a: u32, b: u32) -> u32 {
  return a + b;
}

// without return keyword
fn subtract(a: i32, b: i32) -> i32 {
  a - b
}

fn min_max(a: i32, b: i32) -> (i32, i32) {
  if a > b {
    (b, a)
  } else {
    (a, b)
  }
}