use my_rust_algo::solutions::easy::remove_duplicates::{
    remove_duplicates_simple, remove_duplicates_vec_approach,
};

fn main() {
    let mut nums = vec![1, 1, 1, 2, 2, 3];
    remove_duplicates_simple(&mut nums);
    println!("{nums:?}");
    remove_duplicates_vec_approach(&mut nums);
    println!("{nums:?}");
}
