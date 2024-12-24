use std::rc::Rc;

enum List {
  Cons(i32, Rc<List>),
  Nil,
}

use List::{Cons, Nil};

fn main() {
  let a = Rc::new(Cons(3, Rc::new(Cons(4, Rc::new(Nil)))));

  println!("count: {}", Rc::strong_count(&a));

  // clone does not perform deep copy
  // it increased reference count
  let b = Cons(3, Rc::clone(&a));
  let c = Cons(4, Rc::clone(&a));

  println!("count: {}", Rc::strong_count(&a));

  {
    let d = Cons(6, Rc::clone(&a));
    println!("count: {}", Rc::strong_count(&a));
  }

  println!("count: {}", Rc::strong_count(&a));
}