use std::ops::Add;

struct Point<T> {
  x: T,
  y: T,
}

impl<T> Add for Point<T> where T: Add<Output = T> {
  type Output = Self;

  fn add(self, rhs: Self) -> Self {
    Point {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
    }
  }
}

fn main() {
  let c1 = Point{x: 5.0, y: 4.0};
  let c2 = Point{x: 2.0, y: 2.0};
  let c3 = c1 + c2;
  println!("{}", c3.x);
}
