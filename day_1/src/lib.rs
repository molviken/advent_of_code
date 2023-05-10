use std::error::Error;
use std::fs;

pub struct Config {
    pub binary: String,
    pub file: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let binary = args[0].clone();
        let file = args[1].clone();

        Ok(Config { binary, file })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file)?;

    let results: Vec<i32> = count_calories(&contents);

    println!("Largest amount:\n{}", get_largest(&results));

    let n = 3;
    let mut results_mut: Vec<i32> = results;
    println!("Largest {} amount:\n{}", n, get_n_largest(&mut results_mut, n));

    Ok(())
}

pub fn count_calories(contents: &str) -> Vec<i32> {
    let mut results: Vec<i32> = Vec::new();
    let mut sum: i32 = 0;

    for line in contents.lines() {
        if line.is_empty() {
            results.push(sum);
            sum = 0;
        } else {
            let calories: i32 = line.parse().unwrap();
            sum += calories;
        }
    }
    results.push(sum);
    results
}

pub fn get_largest(contents: &Vec<i32>) -> i32 {
    let mut largest: i32 = 0;

    for number in contents.iter() {
        if *number > largest {
            largest = *number;
        }
    }
    largest
}

pub fn get_n_largest(contents: &mut Vec<i32>, n: i32) -> i32 {
    let mut total: i32 = 0;

    for i in 0..n {
        let mut largest_temp = 0;
        for number in contents.iter() {
            if *number > largest_temp {
                largest_temp = *number;
            }
        }
        let index = contents.iter().position(|&r| r == largest_temp).unwrap();
        contents.remove(index);
        total += largest_temp;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let contents = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        assert_eq!(vec![6000, 4000, 11000, 24000, 10000], count_calories(contents));
    }

    #[test]
    fn find_largest() {
        let contents = vec![6000, 4000, 11000, 24000, 10000];

        assert_eq!(24000, get_largest(contents));
    }
}
