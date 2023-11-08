
/*
enum Result<T, E> {
  Ok(T),
  Err(E),
}
*/

fn div(a: f64, b: f64) -> Result<f64, String> {
  if b == 0.0 {
    Err(String::from("b cant be 0"))
  } else {
    Ok(a/b)
  }

  // better version but not useful (https://github.com/rust-lang/rust/issues/41620)
  // match b {
  //   0.0 => Err(String::from("b cant be 0")),
  //   _ => Ok(a/b),
  // }
}

fn main() {
  let v1 = div(3.4, 6.9);
  let v2 = div(4.0, 0.0);

  println!("{:?}", v1);
  println!("{:?}", v2);

}