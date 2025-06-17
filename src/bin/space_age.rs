use my_rust_algo::solutions::medium::space_age_simple_approach::{Duration, Earth, Planet};

fn main() -> () {
    let seconds = 1_000_000_000;
    let duration = Duration::from(seconds);
    let output = Earth::years_during(&duration);
    println!("{}", output);
}
