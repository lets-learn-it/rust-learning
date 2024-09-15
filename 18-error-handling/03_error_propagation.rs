use std::fs::File;
use std::io::Error;

fn main() {
  let file = open_file().unwrap();
}

fn open_file() -> Result<File, Error>{
  let file = File::open("error.txt")?;
  Ok(file)
}
