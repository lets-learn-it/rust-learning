
// This is single line comment
/* This is multiline comment.
 * you can span this on multiline
 */
fn main() {
  println!("Hello World");

  // formatted hello, <xyz>
  let greet = format!("Hello, {}, How is {}?", "ABC", "PQR");
  println!("{}", greet);

  // stderr
  eprintln!("Hi, This is stderr");
}
