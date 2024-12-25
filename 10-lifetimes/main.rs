fn main() {
  /////////// EXAMPLE 01 ///////////////
  let string1 = String::from("abcd");
  let string2 = String::from("pqr");

  // result1 will have smallest lifetime between string1 & string2
  let result1 = longest(string1.as_str(), string2.as_str());
  println!("The longest string is `{}`", result1);

  /////////// EXAMPLE 02 ///////////////
  let string3 = String::from("pqrs");
  let result2: &str;
  {
    let string4 = String::from("wvxyz");

    // result2 will have smallest lifetime between string3 & string4
    // that is of string4
    result2 = longest(string3.as_str(), string4.as_str());
    println!("The longest string is `{}`", result2);
  }
  // borrowed value does not live long enough
  // println!("The longest string is `{}`", result2);

  /////////// EXAMPLE 03 ///////////////
  let string5 = String::from("pqrs");
  let result3: &str;
  {
    let string6 = String::from("wvxyz");

    // result3 will have smallest lifetime between string3 & string4
    // that is of string4
    result3 = fake_longest(string5.as_str(), string6.as_str());
  }
  println!("The fake_longest string is `{}`", result3);
}

// &i32         // a reference
// &'a i32      // a reference with an explicit lifetime
// &'a mut i32  // a mutable reference with an explicit lifetime
// annotations does not change lifetimes, they just give relationships
// it helps compiler
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}

fn fake_longest<'a>(x: &'a str, y: &str) -> &'a str {
  x
}
