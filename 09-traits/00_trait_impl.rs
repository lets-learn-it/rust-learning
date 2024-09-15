// define trait
trait Animal {
  fn speak(&self) -> &'static str;
}

// define type
struct Dog {
  name: &'static str,
}

impl Dog {
  fn new(name: &'static str) -> Self {
    Dog{name: name}
  }
}



// implement Animal for Dog
impl Animal for Dog {
  fn speak(&self) -> &'static str {
    "I am Dog."
  }
}

fn main() {
  let d: Dog = Dog::new("mooku");
  println!("{}", d.speak())
}
