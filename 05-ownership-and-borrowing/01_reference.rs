fn main(){

  let mut x = 40;         // x is owner

  println!("x: {}", x);

  let x1 = &x;            // immutable reference to x

  println!("x1: {}", x1);

  let x2 = &mut x;        // x2 borrowed x

  println!("x2: {}", x2);
  // println!("x: {}", x);   // this will give error

  *x2 += 80;              // I am owner

  println!("x2: {}", x2);
}