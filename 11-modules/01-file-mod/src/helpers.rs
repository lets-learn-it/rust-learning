// pub is required as we will call this function from main.rs
pub fn full_name(first: &str, last: &str) -> String {
    format!("{} {}", first, last)
}

pub mod chore_helper {
    pub fn do_laundry() {
        println!("Doing laundry...");
    }
}
