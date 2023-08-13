
use std::mem;

fn main() {
  /* Integers */
  let age = 25;
  println!("Age: {}, size of age: {}", age, mem::size_of_val(&age));

  let age: usize = 25;
  println!("Age: {}, size of age: {}", age, mem::size_of_val(&age));

  let rsv: u8 = 0x00;
  
}