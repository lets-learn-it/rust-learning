
// Docs: https://doc.rust-lang.org/std/string/struct.String.html#

fn main() {
  // string: &str and String
  let name: &str = "Parikshit";  // string literal. stored in data section
  println!("{}", name);

  let mut first = String::from("Parikshit"); // stored on heap
  println!("{}", first);

  first.push_str(" Patil");
  println!("{} has length {}", first, first.len());
}