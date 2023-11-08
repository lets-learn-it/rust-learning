
enum Conveyance {
  Car(i32),
  Train(i32),
  Air(i32),
}

impl Conveyance {
  fn travel_allowance(&self) -> f32 {
    let allowance: f32 = match self {
      Conveyance::Car(miles) => *miles as f32 * 15.0 * 2.0,
      Conveyance::Train(miles) => *miles as f32 * 20.0 * 2.0,
      Conveyance::Air(miles) => *miles as f32 * 30.0 * 2.0,
    };

    allowance
  }
}

fn main() {
  let p1: Conveyance = Conveyance::Car(200);
  let p2: Conveyance = Conveyance::Train(250);
  let p3: Conveyance = Conveyance::Air(180);

  // println!("p1: {}", p1 as i32);

  println!("p1: {}, p2: {}, p3: {}", p1.travel_allowance(), p2.travel_allowance(), p3.travel_allowance());
}