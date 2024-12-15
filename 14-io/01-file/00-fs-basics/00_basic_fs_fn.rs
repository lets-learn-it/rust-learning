use std::fs;
use std::path::Path;

fn main() {
  let existing_file = "sample.txt";
  let copied_file = "sampledir/anotherdir/sample.txt";
  let remove_copied = true;

  // check if sample file exists
  if fs::exists(existing_file).unwrap() {
    println!("file exists");
  }

  let path = Path::new(copied_file);
  let parent = path.parent().unwrap();

  if !parent.exists() {
    // create target folder
    println!("path {} does not exist. creating new...", parent.to_str().unwrap());
    fs::create_dir_all(parent).unwrap();
  }

  // copy file
  fs::copy(existing_file, copied_file).unwrap();

  if remove_copied {
    fs::remove_dir_all(parent).unwrap();
  }
}
