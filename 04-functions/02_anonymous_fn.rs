fn main() {
  let x = 5;

  let square1 = || println!("The square of variable is {}", x*x);
  square1();

  let square2 = |num: i32| println!("Square of {}: {}", num, num * num);
  square2(x);

  let mut name = "Parikshit".to_string();
  // cannot borrow as mutable
  // anonymous functions can't capture mutable references
  // change it to mutable
  let mut change_name = || name = "Parikshit Patil".to_string();
  change_name();
  println!("Name: {}", name);
}
