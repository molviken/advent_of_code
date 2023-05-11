use std::error::Error;
use std::fs;
// Rock paper scissors battles


#[derive(Clone, Copy)]
enum Action {
    A = 'A' as isize,
    B = 'B' as isize,
    C = 'C' as isize,
}

impl Action {
    fn from_char(i: char) -> Action {
        match i {
            'A' => Action::A,
            'B' => Action::B,
            'C' => Action::C,
            _ => panic!("Invalid action"),
        }
    }
}

enum ExpectedResult {
    X = 'X' as isize,
    Y = 'Y' as isize,
    Z = 'Z' as isize,
}

impl ExpectedResult {
    fn from_char(i: char) -> ExpectedResult {
        match i {
            'X' => ExpectedResult::X,
            'Y' => ExpectedResult::Y,
            'Z' => ExpectedResult::Z,
            _ => panic!("Invalid action"),
        }
    }
}

struct Player {
    action: Action,
    value: u32,
    weakness: Action,
}

impl Player {
    fn from_action(action: Action) -> Player {
        let weakness: Action;
        let value: u32;
        match action {
            Action::A => {
                value = 1;
                weakness = Action::B;
            },
            Action::B => {
                value = 2;
                weakness = Action::C;
            }
            Action::C => {
                value = 3;
                weakness = Action::A;
            }
        }
        Player {
            action: action.clone(),
            value,
            weakness,
        }
    }

    fn from_player_expected(player_action: Action, result: ExpectedResult) -> Player {
        match result {
            ExpectedResult::X => {
                match player_action {
                    Action::A => Player::from_action(Action::C),
                    Action::B => Player::from_action(Action::A),
                    Action::C => Player::from_action(Action::B),
                }
            },
            ExpectedResult::Y => Player::from_action(player_action),
            ExpectedResult::Z => {
                match player_action {
                    Action::A => Player::from_action(Action::B),
                    Action::B => Player::from_action(Action::C),
                    Action::C => Player::from_action(Action::A),
                }
            },
        }
    }
}


pub fn get_file_path(args: &[String]) -> Result<String, &'static str> {
    if args.len() < 2 {
        return Err("not enough arguments");
    }

    Ok(args[1].clone())
}

pub fn run(input_file: &str) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(input_file)?;

    let score = get_total_score(&contents);
    println!("Result: {}", score);

    Ok(())
}

fn fight(p1: Player, p2: Player) -> u32 {
    let result: u32;
    if p1.action as u32 == p2.weakness as u32 {
        result = 0;
    } else if p1.weakness as u32 == p2.action as u32 {
        result = 6;
    } else {
        result = 3;
    }
    result + p2.value
}

fn get_total_score(input: &str) -> u32 {
    let mut total: u32 = 0;
    for line in input.lines() {
        total += get_single_score(line);
    }
    total
}

fn get_single_score(line: &str) -> u32 {
    let mut split = line.split_whitespace();

    let left = split.next().unwrap().as_bytes()[0];
    let right = split.next().unwrap().as_bytes()[0];

    let p1 = Player::from_action(Action::from_char(left as char));
    let p2 = Player::from_player_expected(p1.action, ExpectedResult::from_char(right as char));

    fight(p1, p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_win() {
        let input = "A Y";

        assert_eq!(8, get_single_score(input));
    }

    #[test]
    fn single_lose() {
        let input = "C Y";

        assert_eq!(2, get_single_score(input));
    }

    #[test]
    fn single_draw() {
        let input = "B Y";

        assert_eq!(5, get_single_score(input));
    }

    #[test]
    fn sample_input() {
        let input = "\
A Y
B X
C Z";

        assert_eq!(15, get_total_score(input));
    }
}
