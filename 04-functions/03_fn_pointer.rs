fn max(x: i32, y: i32) -> i32 {
  if x > y {x} else {y}
}

fn add(x: i32, y: i32) -> i32 {
  x + y
}

fn sub(x: i32, y: i32) -> i32 {
  x - y
}

fn op(f: fn(i32, i32) -> i32, x: i32, y: i32) -> i32 {
  f(x, y)
}

fn main() {
  let f: fn(i32, i32) -> i32 = max;   // function pointer
  println!("Max of two values: {}", f(4,8));

  let a = add;
  let s = sub;

  // 6 - 4 + 7
  println!("6 - 4 + 7: {}", op(a, op(s, 6, 4), 7));

  // anonymous function
  println!("3 + 4: {}", op(|a: i32, b: i32| a+b, 3, 4));
}