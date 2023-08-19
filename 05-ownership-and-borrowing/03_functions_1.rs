
fn main() {
  let s1 = String::from("hello"); // on heap

  takes_ownership(s1);     // s1's value moved to function
  // println!("{}", s1);      // s1 is not owner of string

  let s2 = String::from("World");
  let s2 = takes_ownership_and_return(s2);   // previous s2 is lost so new s2
  println!("{}", s2);

  let x = 5;  // on stack, implemtns copy

  makes_copy(x);
}

fn takes_ownership(s: String) {
  println!("{}", s);
}

fn takes_ownership_and_return(s: String) -> String {
  println!("{}", s);

  s

  // We have to return String 
  // so that we can use it again in main
}

fn makes_copy(i: i32) {
  println!("{}", i);
}
