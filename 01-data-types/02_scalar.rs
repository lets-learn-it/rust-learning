
use std::mem;

fn main() {
  /* Integers */
  let age = 25;
  println!("Age: {}, size of age: {}", age, mem::size_of_val(&age));

  let age: usize = 25;
  println!("Age: {}, size of age: {}", age, mem::size_of_val(&age));

  let rsv: u8 = 0x00;
  println!("rsv: {}, size of rsv: {}", rsv, mem::size_of_val(&rsv));

  // floats
  let pi: f64 = 3.1415926535;
  let p: f32 = 255.0;
  println!("Pi: {}, size of pi: {}", pi, mem::size_of_val(&pi));
  println!("P: {}, size of p: {}", p, mem::size_of_val(&p));

  // type cast
  let pi_int = pi as u8;
  println!("Pi as u8: {}, size of pi_int: {}", pi_int, mem::size_of_val(&pi_int));

  // Boolean
  let is_it_true: bool = true;
  println!("Is it true: {}, size of is_it_true: {}", is_it_true, mem::size_of_val(&is_it_true));
}
