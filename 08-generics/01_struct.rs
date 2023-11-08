#[derive(Debug)]
struct Point<T, P> {
  x: T,
  y: P,
}

fn main() {
  let p1: Point<i32, i32> = Point{x: 5, y: 5};
  let p2: Point<f32, f32> = Point{x: 9.5, y: 5.6};
  let p3: Point<f32, i32> = Point{x: 9.5, y: 5};

  println!("{:?}", p1);
  println!("{:?}", p2);
  println!("{:?}", p3);
}