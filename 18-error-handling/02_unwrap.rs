use std::fs::File;

fn main() {
  // unwrap will panic if there is error
  // let file = File::open("error.txt").unwrap();

  // customize error
  let file = File::open("error.txt").expect("error opening the file");
}
