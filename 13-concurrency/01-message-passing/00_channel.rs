use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
  // mpsc - multiple producer, single consumer
  let (tx, rx) = mpsc::channel();

  // create new thread & move transmitting end to it.
  // spawed thread needs to own the tranmitter to be able to send msgs.
  thread::spawn(move || {
    let val = String::from("hi");
    tx.send(val).unwrap();

    // if tried to print vals, will give error,
    // value borrowed after move. send moved value.

    // lets send multiple values
    let vals = vec![
      String::from("hi"),
      String::from("from"),
      String::from("the"),
      String::from("thread"),
    ];

    for val in vals {
      tx.send(val).unwrap();
      thread::sleep(Duration::from_secs(1));
    }
  });

  // receive message in main thread
  // recv() will block thread until value is sent
  // try_recv() doesn't block. It will return Result<T, E> & Ok value holding msg if available.
  let received = rx.recv().unwrap();
  println!("Got: {}", received);

  // using loop for receiving
  for val in rx {
    println!("Got: {}", val);
  }
}
