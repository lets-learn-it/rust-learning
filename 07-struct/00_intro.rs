
struct User {
  active: bool,
  username: String,
  email: String,
  sign_in_count: u64,
}

fn main() {
  let user: User = User{
    active: true,
    username: String::from("PSKP_95"),
    email: String::from("patilxxxx@gmail.com"),
    sign_in_count: 10,
  };

  println!("Username: {}, email: {}", user.username, user.email);
}