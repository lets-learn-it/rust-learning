use std::collections::BTreeSet;

fn main() {
  let mut bts1 = BTreeSet::new();
  let mut bts2 = BTreeSet::new();

  bts1.insert(1);
  bts1.insert(56);
  bts1.insert(34);
  bts1.insert(23);
  bts1.insert(1);        // 1 added twice

  bts2.insert(10);
  bts2.insert(56);
  bts2.insert(34);
  bts2.insert(69);

  println!("{:?}", bts1);

  // iterate
  for val in bts1.iter() {
    print!("{} ", val);
  }
  println!();

  // remove
  bts1.remove(&1);
  println!("{:?}", bts1);

  // intersection of bts1 & bts2
  let bts3 = bts1.intersection(&bts2);
  let bts4 = &bts1 & &bts2;         // short
  println!("{:?}", bts3);
  println!("{:?}", bts4);

  // union of bts1 & bts2
  let bts5 = bts1.union(&bts2);
  let bts6 = &bts1 | &bts2;         // short
  println!("{:?}", bts5);
  println!("{:?}", bts6);

  // difference of bts1 & bts2
  let bts7 = bts1.difference(&bts2);
  let bts8 = &bts1 - &bts2;         // short
  println!("{:?}", bts7);
  println!("{:?}", bts8);
}
