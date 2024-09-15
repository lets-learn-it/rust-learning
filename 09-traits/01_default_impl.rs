
trait Overview {
  fn overview(&self) -> String {
    format!("This is a default implementation.")
  }
}

struct Course {
  name: &'static str,
}

impl Overview for Course {
  fn overview(&self) -> String {
    format!("{}", self.name)
  }
}

struct AnotherCourse {
  name: &'static str,
}

impl Overview for AnotherCourse{}

fn main() {
  let c1 = Course{name: "Marathi"};
  let c2 = AnotherCourse{name: "Marathi"};

  println!("{}", c1.overview());
  println!("{}", c2.overview());
}
