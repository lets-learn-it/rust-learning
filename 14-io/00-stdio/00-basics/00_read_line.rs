fn main() {
  let mut str: String = String::new();

  std::io::stdin()
    .read_line(&mut str)
    .expect("failed to read input");

  println!("Input: {}", str);

  // parse input into float
  let n: f64 = str.trim().parse().expect("invalid input");

  println!("parsed float: {}", n);
}
