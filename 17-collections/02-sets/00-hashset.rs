use std::collections::HashSet;

fn main() {
  let mut hs1 = HashSet::new();
  let mut hs2 = HashSet::new();

  hs1.insert(1);
  hs1.insert(56);
  hs1.insert(34);
  hs1.insert(23);
  hs1.insert(1);        // 1 added twice

  hs2.insert(10);
  hs2.insert(56);
  hs2.insert(34);
  hs2.insert(69);

  println!("{:?}", hs1);

  // iterate
  for val in hs1.iter() {
    print!("{} ", val);
  }
  println!();

  // remove
  hs1.remove(&1);
  println!("{:?}", hs1);

  // intersection of hs1 & hs2
  let hs3 = hs1.intersection(&hs2);
  let hs4 = &hs1 & &hs2;         // short
  println!("{:?}", hs3);
  println!("{:?}", hs4);

  // union of hs1 & hs2
  let hs5 = hs1.union(&hs2);
  let hs6 = &hs1 | &hs2;         // short
  println!("{:?}", hs5);
  println!("{:?}", hs6);

  // difference of hs1 & hs2
  let hs7 = hs1.difference(&hs2);
  let hs8 = &hs1 - &hs2;         // short
  println!("{:?}", hs7);
  println!("{:?}", hs8);
}
