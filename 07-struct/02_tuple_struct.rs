// individual fields dont have names but struct have

struct Numbers(i32, i32);

impl Numbers {
  fn greater(&self) -> i32 {
    if self.0 >= self.1 { self.0 } else { self.1 }
  }
}

fn main() {
  let nums: Numbers = Numbers(32,64);
  println!("Values of fields: {} & {}", nums.0, nums.1);

  println!("biggest of two: {}", nums.greater());
}