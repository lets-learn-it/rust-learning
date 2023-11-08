
enum Conveyance {
  Car = 15,
  Train = 20,
  Air = 30,
}

impl Conveyance {
  fn travel_allowance(&self, miles: i32) -> f32 {
    let allowance: f32 = match self {
      Conveyance::Car => miles as f32 * 150.0 * 2.0,
      Conveyance::Train => miles as f32 * 200.0 * 2.0,
      Conveyance::Air => miles as f32 * 300.0 * 2.0,
    };

    allowance
  }
}

fn main() {
  let p1: Conveyance = Conveyance::Car;
  let p2: Conveyance = Conveyance::Train;
  let p3: Conveyance = Conveyance::Air;

  // println!("p1: {}", p1 as i32);

  println!("p1: {}, p2: {}, p3: {}", p1.travel_allowance(200), p2.travel_allowance(250), p3.travel_allowance(180));
}