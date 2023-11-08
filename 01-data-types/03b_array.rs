
 fn main() {
  // Array
  let arr = [1,2,3,4,5];
  println!("Lenght of Array: {}, First element: {}", arr.len(), arr[0]);
  println!("{:?}", arr);         // print whole array

  // array slice (it is reference so cant be modified)
  println!("{:?}", &arr[1..=3]); // print part of array

  // check memory used by array
  println!("The array is occupying {} bytes.", std::mem::size_of_val(&arr));

  // get value if exists
  let val: Option<&i32> = arr.get(100);
  println!("{:?}", val);

  // initialize array
  let brr = [3; 5];              // number 3 five times
  println!("{}", brr[0]);
  println!("{:?}", brr)
}