use std::error::Error;
use std::fs;

pub fn get_file_path(args: &[String]) -> Result<String, &'static str> {
    if args.len() < 2 {
        return Err("not enough arguments");
    }

    Ok(args[1].clone())
}

pub fn task_1(input_file: &str) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(input_file)?;

    let score = get_total_priority_multiple_backpacks(&contents);
    println!("Result: {}", score);

    Ok(())
}

pub fn task_2(input_file: &str) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(input_file)?;

    let score = get_total_priority_multiple_groups(&contents).unwrap_or_else(|err| {
        println!("Failed getting total score: {}", err);
        0
    });

    println!("Result: {}", score);

    Ok(())
}

fn get_priority(c: char) -> Result<u32, &'static str> {
    match c {
        'a'..='z' => Ok(c as u32 - 96),
        'A'..='Z' => Ok(c as u32 - 38),
        _ => {
            Err("Invalid input")
        }
    }
}

// TASK 1
fn get_similarities_in_backpack(backpack: &str) -> Vec<char> {
    let mut similarities: Vec<char> = Vec::new();
    let (left, right) = backpack.split_at(backpack.len()/2);
    for c in left.chars() {
        if right.contains(c) && !similarities.contains(&c) {
            similarities.push(c);
        }
    }
    similarities
}

fn get_total_priority_single_backpack(similarities: &Vec<char>) -> u32 {
    let mut total = 0;
    for similarity in similarities {
        total += get_priority(*similarity).unwrap();
    }
    total
}

fn get_total_priority_multiple_backpacks(input: &str) -> u32 {
    let mut total = 0;
    for line in input.lines() {
        let similarities_backpack: Vec<char> = get_similarities_in_backpack(line);
        total += get_total_priority_single_backpack(&similarities_backpack);
    }
    total
}

// TASK 2
fn get_lines_count(input: &str) -> u32{
    let mut count = 0;
    for _ in input.lines() {
        count += 1;
    }
    count
}

fn get_badge_group(bp1:  &str, bp2:  &str, bp3:  &str) -> Result<char, &'static str> {
    for c in bp1.chars() {
        let has12: bool;
        let has13: bool;

        if bp2.contains(c) {
            has12 = true;
        } else {
            has12 = false;
        }

        if bp3.contains(c) {
            has13 = true;
        } else {
            has13 = false;
        }

        if has12 && has13 {
            return Ok(c)
        }
    }
    Err("Unable to find the badge!")
}

fn get_total_priority_multiple_groups(input: &str) -> Result<u32, &'static str> {
    let mut total = 0;
    let len: u32 = get_lines_count(input);
    println!("Len: {}", len);

    if len % 3 != 0 {
        return Err("Number of backpacks not divisible by 3")
    }
    let mut iter = input.lines();
    let mut counter = 0;
    for _ in (0..len).step_by(3) {
        counter += 1;
        let badge = get_badge_group(iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap()).unwrap_or_else(|err| {
            println!("Invalid badge: {}", err);
            char::default()
        });
        let priority = get_priority(badge).unwrap_or_else(|err| {
            println!("Unable to get priority: {}", err);
            0
        });
        if !priority == 0 {
            return Err("Invalid group!")
        }
        total += priority;
    }
    println!("Counter: {}", counter);

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_priority() {
        let input0: char = 'L';
        let input1: char = 'A';
        let input2: char = 'Z';
        let input3: char = 'a';
        let input4: char = 'z';
        assert_eq!(38, get_priority(input0).unwrap());
        assert_eq!(27, get_priority(input1).unwrap());
        assert_eq!(52, get_priority(input2).unwrap());
        assert_eq!(1, get_priority(input3).unwrap());
        assert_eq!(26, get_priority(input4).unwrap());
    }

    #[test]
    fn sample_input_task1() {
        let input = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(157, get_total_priority_multiple_backpacks(input));
    }

    #[test]
    fn sample_input_task2() {
        let input = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(70, get_total_priority_multiple_groups(input).unwrap_or_else(|_| {0}));
    }

}
