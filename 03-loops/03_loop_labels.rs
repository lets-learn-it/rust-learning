
fn main() {
  'outer: for i in 0..5 {
    'inner: for j in 0..5 {
      println!("i: {}, j: {}", i, j);
      if j >= 2 {
        break 'outer; // Breaks out of the outer loop
      }
    }
  }
}
