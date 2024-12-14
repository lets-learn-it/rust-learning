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

fn main() {
  let a = City{city: String::from("A"), population: 100};
  let b = City{city: String::from("B"), population: 90};
}
