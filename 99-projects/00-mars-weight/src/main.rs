use std::io::{self, Write};

fn main() {
    print!("Enter your weight (Kg): ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    println!("You entered: {}", input);

    let weight: f32 = input.trim().parse().unwrap();


    let result: f32 = calculate_weight_on_mars(weight);
    println!("Weight on mars: {}", result)
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}
