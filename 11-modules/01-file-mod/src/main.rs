mod helpers;

use helpers::chore_helper;

fn main() {
    let full_name = helpers::full_name("John", "Doe");
    println!("Full name: {}", full_name);

    helpers::chore_helper::do_laundry();

    chore_helper::do_laundry();
}
