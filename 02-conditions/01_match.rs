
fn main() {
  let num = 20;

  match num {
    10 | 20 | 30 => println!("10 or 20 or 30"),
    20 => println!("20"),  // unreachable pattern
    100 => println!("its 100"),
    _ => println!("just default case"),
  }
}
