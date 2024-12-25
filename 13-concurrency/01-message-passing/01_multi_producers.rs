use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
  let (tx, rx) = mpsc::channel();
  let tx1 = tx.clone();

  thread::spawn(move || {
    let vals = vec![
      String::from("01: hi"),
      String::from("01: from"),
      String::from("01: the"),
      String::from("01: thread"),
    ];

    for val in vals {
      tx.send(val).unwrap();
      thread::sleep(Duration::from_secs(1));
    }
  });

  thread::spawn(move || {
    let vals = vec![
      String::from("02: This"),
      String::from("02: is"),
      String::from("02: second"),
      String::from("02: thread"),
    ];

    for val in vals {
      tx1.send(val).unwrap();
      thread::sleep(Duration::from_secs(1));
    }
  });

  // using loop for receiving
  for val in rx {
    println!("Got: {}", val);
  }
}
