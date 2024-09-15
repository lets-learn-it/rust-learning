use std::fs::File;
use std::io::ErrorKind;

fn main() {
  let file = File::open("error.txt");

  let file = match file {
    Ok(file) => file,
    Err(e) => match e.kind() {
      ErrorKind::NotFound => panic!("file does not exists"),
      _ => panic!("something else"),
    },
  };
}
