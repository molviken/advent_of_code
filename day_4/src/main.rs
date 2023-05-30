use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt")
        .expect("File doesn't exist!");

    println!("Result of task 1: {}", day_4::task_1(&content));
    println!("Result of task 2: {}", day_4::task_2(&content));
}
