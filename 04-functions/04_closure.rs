struct City {
  city: String,
  population: u64,
}

fn sort_pop(city: &mut Vec<City>) {
  city.sort_by_key(pop_helper);
}

fn pop_helper(pop: &City) ->u64 {
  pop.population
}

fn adder(x: i32) -> impl Fn(i32) -> i32 {
  move |y| x + y
}

fn main() {
  // ex 1
  let a = City{city: String::from("A"), population: 100};
  let b = City{city: String::from("B"), population: 90};

  let mut cities = vec![a, b];
  sort_pop(&mut cities);

  for city in cities {
    println!("City: {}, Population: {}", city.city, city.population);
  }

  // ex 2
  let add10 = adder(10);
  println!("{}", add10(5));
}
