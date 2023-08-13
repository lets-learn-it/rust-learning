
fn main() {
  let age = 20;

  if age > 100 {
    println!("You may be dead");
  } else if age > 18 {
    println!("Allowed to drive and vote");
  } else {
    println!("No");
  }

  // if as an expression
  let quote = if age > 50 {
    "old fish"
  } else {
    "young blood"
  };

  println!("\"{}\"", quote);
}
