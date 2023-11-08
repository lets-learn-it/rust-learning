// resizable

fn main() {
  let mut num_vec: Vec<i32> = vec![3,4,5,6,6,34,45,2,234];
  let zero_vec = vec![0; 10];

  // printing
  println!("first element: {}", num_vec[0]);
  println!("Original: {:?} with length: {}", num_vec, num_vec.len());
  println!("Zero Vec: {:?}", zero_vec);

  // update
  num_vec[4] = 54;
  println!("After changing 4th element: {:?}", num_vec);

  // vector slice
  let sub_vec: &&[i32] = &&num_vec[2..5];
  println!("Vector slice: {:?}", sub_vec);

  // get Option
  let val: Option<&i32> = num_vec.get(100);
  println!("val: {:?}", val);

  // push value at end
  num_vec.push(45);
  num_vec.push(450);
  println!("After pushing: {:?}", num_vec);

  // remove
  num_vec.remove(8);
  println!("After removing: {:?}", num_vec);

  // check if contain
  println!("value 2 exists: {}", num_vec.contains(&2));
  println!("value 10 exists: {}", num_vec.contains(&10));
}