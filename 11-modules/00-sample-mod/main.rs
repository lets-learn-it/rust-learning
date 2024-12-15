mod helpers {
  pub fn full_name(first_name: &str, last_name: &str) -> String {
    format!("{} {}", first_name, last_name)
  }

  // this module should be public
  // as we will call this function from main function
  pub mod greeter {
    pub fn greet() {
      println!("Hello from greeter module!");
    }
  }
}

fn main() {
  let full_name = helpers::full_name("John", "Doe");
  println!("Full name: {}", full_name);

  helpers::greeter::greet();
}
