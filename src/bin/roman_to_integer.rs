use my_rust_algo::solutions::easy::roman_to_integer::{int_to_roman, roman_to_int};

fn main() {
    let number = 3;
    println!("{}", int_to_roman(number).unwrap());
    println!("{}", roman_to_int(String::from("III")).unwrap());
}
