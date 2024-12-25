use std::thread;
use std::time::Duration;

fn main() {
  thread::spawn(|| {
    for i in 1..10 {
      println!("Hi number {} from the spawned thread!", i);
      thread::sleep(Duration::from_millis(10));
    }
  });

  for i in 1..5 {
    println!("Hi number {} from main thread!", i);
    thread::sleep(Duration::from_millis(10));
  }

  // Note: When the main thread of rust program completes, all spawned threads are shut down,
  // whether or not they have finished running.
}
