use std::thread;

// move passed to thread::spawn will take ownership of values it uses from environmnet
// thus transferring ownership of those values from one thread to another.

fn main() {
  let v = vec![1,2,3];

  let handle = thread::spawn(move || {
    println!("vector: {:?}", v);
  });

  // What if main thread drop(v) before
  // spawned thread can complete

  handle.join().unwrap();
}
