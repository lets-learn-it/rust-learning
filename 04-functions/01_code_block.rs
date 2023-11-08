fn main() {
  let formatted_text: String = {
    let first_name = "Parikshit";
    let last_name = "Patil";
    format!("My name is {} {}", first_name, last_name)
  };

  println!("Formatted text: {}", formatted_text);
}