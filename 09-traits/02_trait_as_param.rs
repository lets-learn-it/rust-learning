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

fn say_something(animal: &impl Animal) {
  println!("{}", animal.speak())
}

fn say_more<T: Animal> (item: &T) {
  println!("{}", item.speak())
}

fn main() {
  let d: Dog = Dog::new("mooku");
  say_something(&d);
  say_more(&d);
}


// Notes
// passed i & j can be different types
// fn say(i: &impl Animal, j: &impl Animal)
// passed i & j should be of same type
// fn say<T: Animal>(i: &T, j:&T)
// multiple trains allowed
// fn say(i: &impl Animal + Robot)
// fn say<T: Animal + Robot>(i: &T)
