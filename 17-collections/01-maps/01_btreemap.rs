use std::collections::BTreeMap;

fn main() {
  let mut btm = BTreeMap::new();

  btm.insert(1, 1);
  btm.insert(5, 3);
  btm.insert(2, 6);
  let old = btm.insert(2, 60);

  println!("{:?}", btm);
  println!("old value: {:?}", old);

  // check if key exists
  println!("{}", btm.contains_key(&5));

  // get if key exists
  println!("{:?}", btm.get(&5));

  // remove
  let removed = btm.remove(&5);
  println!("{:?}", btm);
  println!("removed value: {:?}", removed);

  // iterate
  for (key, value) in &btm {
    println!("Key: {key}, Value: {value}")
  }

  // clear hasbtmap
  btm.clear();
  println!("{}", btm.is_empty());
}
