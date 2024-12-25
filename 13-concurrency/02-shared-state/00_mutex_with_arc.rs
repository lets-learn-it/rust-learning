use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
  // Arc = Atomic reference counting. it is like Rc but for concurrent access.
  let counter = Arc::new(Mutex::new(0));
  let mut handles = vec![];

  for _ in 0..10 {
    // clone with increase Arc counter
    let counter = Arc::clone(&counter);

    // spawn new thread which will move counter (shadowed variable)
    let handle = thread::spawn(move || {
      // lock returns LockResult<MutexGuard>
      let mut num = counter.lock().unwrap();

      *num += 1;

      // MutexGuard dropped and mutex is unlocked
    });
    handles.push(handle);
  }

  for handle in handles {
    handle.join().unwrap();
  }

  println!("Result: {}", *counter.lock().unwrap());
}
