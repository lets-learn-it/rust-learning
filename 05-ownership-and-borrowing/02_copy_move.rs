
fn main() {
  let s1 = String::from("hello");
  let s2 = s1;       // s1 is invalid now.
  let s3 = s2.clone();

  // println!("{}, world!", s1);  // this will give error
  println!("{}, world!", s2);  // this will not.
  println!("{}, world!", s3);
}