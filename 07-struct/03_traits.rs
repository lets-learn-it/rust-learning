// trait is like interface
trait GeneralInfo {
  fn info(&self) -> (&str, u8, char);
  fn country(&self) -> &str {
    "default contry"
  }
}

#[derive(Debug)]
struct Person {
  nationality: String,
  name: String,
  age: u8,
  gender: char,
  salary: u32
}

impl GeneralInfo for Person {
  fn info(&self) -> (&str, u8, char) {
    (&self.name, self.age, self.gender)
  }

  fn country(&self) -> &str {
    &self.nationality
  }
}

struct Student {
  nationality: String,
  name: String,
  age: u8,
  gender: char,
  marks: i8
}

impl GeneralInfo for Student {
  fn info(&self) -> (&str, u8, char) {
    (&self.name, self.age, self.gender)
  }

  // not implementing contry so default will be used.
  // fn country(&self) -> &str {
  //   &self.nationality
  // }
}

fn main() {
  let p1: Person = Person {
    nationality: String::from("indian"),
    name: String::from("Parikshit"),
    age: 25,
    gender: 'M',
    salary: 1000
  };

  let s1: Student = Student {
    nationality: String::from("indian"),
    name: String::from("Parikshit"),
    age: 25,
    gender: 'M',
    marks: 95
  };

  print_info(&p1);
  print_info(&s1);
}

fn print_info(x: &dyn GeneralInfo) {
  println!("{:?}", x.info());
  println!("{}", x.country());
}