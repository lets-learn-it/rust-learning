#[derive(Debug)]
struct Point<T, P> {
  x: T,
  y: P,
}

impl<T: std::fmt::Debug, P: std::fmt::Debug> Point<T, P> {
  fn print(&self) {
    println!("x: {:?}, y: {:?}", self.x, self.y);
  }
}

// impl<T, P> Point<T, P>
// where T: std::fmt::Debug, P: std::fmt::Debug{
//   fn print(&self) {
//     println!("x: {:?}, y: {:?}", self.x, self.y);
//   }
// }

fn main() {
  let p1: Point<i32, i32> = Point{x: 5, y: 5};
  let p2: Point<f32, f32> = Point{x: 9.5, y: 5.6};
  let p3: Point<f32, i32> = Point{x: 9.5, y: 5};

  p1.print();
  p2.print();
  p3.print();
}