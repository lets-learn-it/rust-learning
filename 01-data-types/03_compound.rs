

fn main() {
  // Tuple
  let t = ("parikshit", 25);
  let name = t.0;
  let age = t.1;

  println!("Name: {}, Age: {}", t.0, t.1);
  println!("Name: {}, Age: {}", name, age);

  // Array
  let arr = [1,2,3,4,5];
  println!("{}", arr[0]);

  // initialize array
  let brr = [3; 5];
  println!("{}", brr[0]);
}