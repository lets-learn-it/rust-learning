
fn print_vec(v: &Vec<i32>) {
  for elem in v.iter() {
    print!("{} ", elem);
  }
  println!()
}

fn main() {
  // creation
  let mut v1 = Vec::new();
  let mut v2 = vec![1,2,3];
  let mut v3 = Vec::from([1,2,3,4]);

  v1.push(3);
  v1.extend([1,2,4]);
  v1.insert(2, 34);
  v1.remove(0);

  let num = v1.pop();  // returns Option<T>
  println!("{:?}", num);

  print_vec(&v1);

  println!("{}", v1.len());
  println!("{}", v2.is_empty());
}
