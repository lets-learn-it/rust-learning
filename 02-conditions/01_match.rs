
fn main() {
  let num = 200;

  match num {
    10 | 20 | 30 => println!("10 or 20 or 30"),
    20 => println!("20"),  // unreachable pattern
    100 => println!("its 100"),
    100..=1000 => println!("between 100 and 1000"),
    _ => println!("just default case"),
  }

  // match expression
  let age = 20;
  let quote = match age {
    40..=100 => "old fish",
    10..=39 => "young blood",
    0..=9 => "chill kid",
    _ => "are you dead?"
  };

  println!("\"{}\"", quote);
}
