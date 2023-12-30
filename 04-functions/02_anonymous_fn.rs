fn main() {
  let x = 5;

  let square1 = || println!("The square of variable is {}", x*x);
  square1();

  let square2 = |num: i32| println!("Square of {}: {}", num, num * num);
  square2(x);
}