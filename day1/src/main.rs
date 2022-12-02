use std::fs;

fn main() {
    let input = fs::read_to_string("input")
        .expect("Failed to read input!");
    let list_of_calories = input.lines();

    let mut total_calories_current_elf = 0;
    let mut total_calories_best_elf = 0;

    for line in list_of_calories {
        // a new elf
        if line.is_empty() {
            if total_calories_current_elf > total_calories_best_elf {
                total_calories_best_elf = total_calories_current_elf;
            }
            total_calories_current_elf = 0;
            continue
        }
        let number: u64 = line.parse().unwrap();
        // update calories
        total_calories_current_elf += number;
        // update highest_calories and best_elf
    }
    println!("{total_calories_best_elf}");
}

// track calories of top three elves. Return some of those calories
// https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html
// A priority queue implemented with a binary heap.
