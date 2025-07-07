use my_rust_algo::solutions::easy::difference_of_squares::*;

fn main() {
    let n = 1000;
    use std::time::Instant;
    let now = Instant::now();

    {
        sum_of_squares(n);
    }

    let elapsed = now.elapsed();
    println!("Time elapsed: {elapsed:?}");
}
