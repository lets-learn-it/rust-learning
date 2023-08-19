
fn main() {
  let numbers = [1,22,3,4,5,6,7];

  for num in numbers.iter() {
    print!("{} ", num);
  }

  println!("");

  for (index, num) in numbers.iter().enumerate() {
    print!("Index: {}, Value: {} \n", index, num);
  }

  println!("");

  for num in numbers {
    print!("{} ", num);
  }

  println!("");

  for i in 0..5 {
    print!("{} ", i);
  }
}