use std::error::Error;
use std::fs;

pub fn get_file_path(args: &[String]) -> Result<String, &'static str> {
    if args.len() < 2 {
        return Err("not enough arguments");
    }

    Ok(args[1].clone())
}

pub fn run(input_file: &str) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(input_file)?;

    let score = get_total_priority_multiple_backpacks(&contents);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_priority() {
        let input: char = 'z';
        assert_eq!(38, get_priority(input).unwrap());
    }

    #[test]
    fn sample_input() {
        let input = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(157, get_total_priority_multiple_backpacks(input));
    }
}
