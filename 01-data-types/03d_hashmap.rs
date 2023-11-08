use std::collections::HashMap;

fn main() {
  let mut person: HashMap<&str, i32> = HashMap::new();

  person.insert("Parikshit", 25);
  person.insert("P1", 40);
  person.insert("P2", 43);

  // get
  println!("Age: {:?}", person.get("Parikshit"));
  println!("Age: {:?}", person.get("xyz"));

  // contains
  println!("contains: {:?}", person.contains_key("xyz"));
  println!("contains: {:?}", person.contains_key("P2"));

  // if exist, do not update
  person.entry("xyz").or_insert(54);
  person.entry("Parikshit").or_insert(26);

  // print all key value
  for (name, age) in &person {
    println!("name: {}, age: {}", name, age);
  }
}