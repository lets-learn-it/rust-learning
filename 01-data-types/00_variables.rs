
fn main() {
  let name = "Parikshit";
  println!("Name: {}", name);

  // mut
  let mut age: i32 = 25;
  println!("Age: {}", age);
  age = 26;
  println!("Age: {}", age);


  // let me redeclare name
  // It is called shodowing
  let name = "PSKP";
  println!("Name: {}", name);

  {
    let name = "PSKP_95";
    println!("Name: {} (inside brackets)", name);
  }

  println!("Name: {}", name);
}