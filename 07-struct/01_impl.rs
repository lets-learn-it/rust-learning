#[derive(Debug)]
struct Person {
  nationality: String,
  name: String,
  age: i32,
  gender: char,
  salary: i32
}

impl Person {
  // Self is person. you can use Person too
  fn new() -> Self {
    Self{
      nationality: String::from("Indian"),
      name: String::from("parikshit"),
      age: 25,
      gender: 'M',
      salary: 1000
    }
  }

  fn compute_taxes(&self) -> f32 {
    self.salary as f32 * 0.3
  }
}

fn main() {
  let p1: Person = Person::new();

  // fields
  println!("Name: {}", p1.name);
  println!("{:?}", p1);

  // both are same
  println!("{} should pay {} taxes.", p1.name, p1.compute_taxes());
  println!("{} should pay {} taxes.", p1.name, Person::compute_taxes(&p1));

  let p2: Person = Person{
    name: String::from("ABC"),
    age: 120,
    ..p1    // gets other fields from p1
  };

  println!("{:?}", p2);
}