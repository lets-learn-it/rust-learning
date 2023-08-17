
fn main() {
  greet();

  println!("Sum: {}", sum(4, 5));

  println!("Subtract: {}", subtract(4, 5));
}

fn greet()  {
  println!("Hello, World");
}

fn sum(a: u32, b: u32) -> u32 {
  return a + b;
}

// without return keyword
fn subtract(a: i32, b: i32) -> i32 {
  a - b
}