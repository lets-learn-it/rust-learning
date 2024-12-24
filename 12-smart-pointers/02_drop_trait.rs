// It is like destructor in cpp
// Rust will call drop method when variable goes out of scope

struct CustomSmartPointer {
  data: String,
}

impl Drop for CustomSmartPointer {
  fn drop(&mut self) {
    println!("Dropping CustomSmartPointer with data `{}`!", self.data);
  }
}

fn main() {
  let c = CustomSmartPointer {
    data: String::from("Parikshit"),
  };
  let d = CustomSmartPointer {
    data: String::from("Patil"),
  };
  let e = CustomSmartPointer {
    data: String::from("Kalpana"),
  };

  // manually drop d
  // explicit destructor call not allowed like d.drop()
  drop(d);

  // e will be dropped here
  // c will be dropped here
  // dropping happens in reverse order.
}