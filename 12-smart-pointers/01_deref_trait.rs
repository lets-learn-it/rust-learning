use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
  // not storing on heap
  // Box stores on heap
  fn new(x: T) -> MyBox<T> {
    MyBox(x)
  }
}

impl<T> Deref for MyBox<T> {
  type Target = T;

  fn deref(&self) -> &T {
    &self.0
  }
}

fn main() {
  let x = 5;
  // reference to x (5)
  let y = &x;

  // need to dereference
  assert_eq!(5, *y);

  let z = Box::new(x);
  let myz = MyBox::new(x);

  // we can dereference Box type
  // as Box is implementing Deref trait
  assert_eq!(5, *z);
  assert_eq!(5, *myz);

  // implicit deref coercion
  // convert reference from 1 type to different type
  // &MyBox<String> -> &String -> &str
  let name = MyBox::new(String::from("Parikshit"));
  hello(&name);
}

fn hello(name: &str) {
  println!("Hello, {}!", name);
}
