
// Docs: https://doc.rust-lang.org/std/string/struct.String.html#

fn main() {
  // string: &str and String
  let name: &str = "Parikshit";  // string literal. stored in data section
  println!("{}", name);          // can't be modified

  let mut first = String::from("Parikshit"); // stored on heap
  let f = name.to_string();                  // from literal
  let last = " Patil".to_string();
  println!("{}", first);

  first.push_str(&last);
  println!("{} has length {}", first, first.len());
  println!("last {}", last);
  println!("f {}", f);

  // comparison using == and !=
  println!("{}", "ONE".to_lowercase() == "one");
}