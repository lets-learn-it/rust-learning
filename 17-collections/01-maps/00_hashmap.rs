use std::collections::HashMap;

fn main() {
  let mut hm = HashMap::new();

  hm.insert(1, 1);
  hm.insert(5, 3);
  hm.insert(2, 6);
  let old = hm.insert(2, 60);

  println!("{:?}", hm);
  println!("old value: {:?}", old);

  // check if key exists
  println!("{}", hm.contains_key(&5));

  // get if key exists
  println!("{:?}", hm.get(&5));

  // remove
  let removed = hm.remove(&5);
  println!("{:?}", hm);
  println!("removed value: {:?}", removed);

  // clear hashmap
  hm.clear();

  println!("{}", hm.is_empty());
}
