// When the owner goes out of scope, the value will be dropped.

fn main() {
  
  {
    let a = 10;
    println!("a: {}", a);
  }

  // println!("a: {}", a);  // a is not in scope
}