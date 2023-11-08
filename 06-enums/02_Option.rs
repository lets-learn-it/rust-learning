// Option enum has 2 variants
// 1. None: failure or lack of value
// 2. Some: generic tuple struct

/*
enum Option<T> {
  None,
  Some(T),
}
*/
fn main() {
  let mut disease: Option<String> = None;
  disease = Some(String::from("diabetes"));

  match disease {
    None => println!("No disease."),
    Some(x) => println!("Some disease. {}", x),
  }

  // get value out of Some
  let f1: Option<f64> = Some(50.5);
  let mut f2: f64 = 16.5;
  f2 = f2 + f1.unwrap();
  println!("f2: {}", f2);

  // built in
  let v1 = vec![2,3,4,5,5,6,7,4];
  println!("{:?}", v1.get(30));
}