fn main() {
  // Tuple
  let t = ("parikshit", 25);
  let name = t.0;
  let age = t.1;

  println!("Name: {}, Age: {}", t.0, t.1);
  println!("{:?}", t);
  println!("Name: {}, Age: {}", name, age);

  let t1 = (1, 2, ("Parikshit", 25));
  let (name1, age1) = t;
  println!("Name {}, Age {}", t1.2.0, t1.2.1);
  println!("Name: {}, Age: {}", name1, age1);

}