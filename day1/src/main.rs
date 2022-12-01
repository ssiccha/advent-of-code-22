use std::fs;

fn main() {
    println!("Hello, Sergio!");
    let _input = fs::read_to_string("input")
        .expect("Failed to read input!");

    let list_of_calories = _input.lines();
    let nr_cals = list_of_calories.count();
    println!("len list {nr_cals}");
    let nr: i32 = "1".parse().unwrap();
    println!("You guessed {nr}");
}
