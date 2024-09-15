use std::collections::BinaryHeap;

fn main() {
  let mut bheap = BinaryHeap::new();

  bheap.push(1);
  bheap.push(19);
  bheap.push(41);
  bheap.push(21);
  bheap.push(12);

  println!("{:?}", bheap);

  bheap.pop();
  println!("{:?}", bheap);

  let ele = bheap.peek();
  println!("{:?}", ele);
  println!("{:?}", bheap);
}