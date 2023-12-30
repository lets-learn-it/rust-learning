
fn main() {
  let s = String::from("Hello");
  without_ownership(&s);         // s is still owner

  let dangle = dangling_reference();
  let no_dangle = no_dangling_reference();

  println!("{}", no_dangle);
}

fn without_ownership(s: &String) {
  println!("{}", s);
}

// This function will give compile time error.
fn dangling_reference() -> &String {
  let s = String::from("World");   // s is owner

  &s

  // &s is returned, so this s is still owner.
  // Which means once function completed, underlying string will be dropped.
}

fn no_dangling_reference() -> String {
  let s = String::from("World");   // s is owner

  s

  // s is returned, so s gives up ownership.
}
