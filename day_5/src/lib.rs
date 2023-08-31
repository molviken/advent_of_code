struct Expedition {
    crates: Vec<Vec<char>>,
    instructions: Vec<Instruction>,
}

struct Instruction {
    amount: u32,
    from: u32,
    to: u32,
}

impl Instruction {
    fn print(&self) {
        println!("move {} from {} to {}", self.amount, self.from, self.to);
    }
}

impl Expedition {
    fn from_string(input: &str) -> Expedition {
        let mut split = false;
        let mut crates_str: String = "".to_owned();
        let mut instr_str: String = "".to_owned();
        for line in input.lines() {
            if line.is_empty() {
                split = true;
            }

            if !split {
                crates_str.push_str(line);
                crates_str.push_str("\n");
                println!("{}", line);
            } else {
                instr_str.push_str(line.trim());
                instr_str.push_str("\n");
            }
        }
        let crates: Vec<Vec<char>> = build_crates(&crates_str);
        let instructions: Vec<Instruction> = build_instructions(&instr_str);
        Expedition {
            crates,
            instructions
        }
    }

    fn reorganize1(&mut self) {
        let Expedition{ ref mut crates, ref instructions } = *self;
        print_crates(crates);
        for instruction in instructions {
            let amount = instruction.amount;
            let from = instruction.from as usize;
            let to = instruction.to as usize;
            for _ in 0..amount {
                let c = *crates[from - 1].last().unwrap();
                crates[to - 1].push(c);
                crates[from - 1].pop();
            }
        }
    }

    fn reorganize2(&mut self) {
        let Expedition{ ref mut crates, ref instructions } = *self;
        print_crates(crates);
        for instruction in instructions {
            let amount = instruction.amount;
            let from = instruction.from as usize;
            let to = instruction.to as usize;
            let mut temp: Vec<char> = Vec::new();
            for _ in 0..amount {
                let c = *crates[from - 1].last().unwrap();
                temp.insert(0, c);
                crates[from - 1].pop();
            }
            crates[to - 1].extend(temp);
        }
    }

    fn get_top_crates(&self) -> String {
        let mut result = String::from("");
        for vec in &self.crates {
            result.push_str(&format!("{}", vec.last().unwrap()));
        }
        result
    }
}

fn print_crates(crates: &Vec<Vec<char>>) {
    for vec in crates {
        for c in vec {
            if c.is_whitespace() {
                print!(" ")
            } else {
                print!("[{}] ", c);
            }
        } println!("");
    } println!("");
}

fn build_crates(input: &str) -> Vec<Vec<char>> {
    const NUM_WHITESPACE_IS_EMPTY: u32 = 4;
    let mut max_index = 0;
    let mut crates: Vec<Vec<char>> = Vec::new();;
    crates.resize(max_index, Vec::new());
    for line in input.lines() {
        if max_index == 0 {
            max_index = (line.len() + 1)/4;
            crates.resize(max_index, Vec::new());
        }
        let mut whitespace_counter = 0;
        let mut index = 0;
        for c in line.chars() {
            if !c.is_whitespace() && !c.is_ascii_uppercase() && !c.is_ascii_lowercase() {
                whitespace_counter = 0;
                continue
            }
            if c.is_whitespace() {
                whitespace_counter += 1;
                if whitespace_counter == NUM_WHITESPACE_IS_EMPTY {
                    index += 1;
                    whitespace_counter = 0;
                }
                continue;
            } else {
                whitespace_counter = 0;
                index += 1;
            }
            crates[index - 1].insert(0, c);
        }
    }
    crates
}

fn build_instructions(input: &str) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            continue
        }
        let numbers_str: Vec<&str> = line.split(|c:char| !c.is_numeric()).collect();
        let mut numbers: Vec<u32> = Vec::new();
        for c in numbers_str {
            if !c.is_empty() {
                numbers.push(c.parse().unwrap());
            }
        }
        let instruction = Instruction {
            amount: numbers[0],
            from: numbers[1],
            to: numbers[2],
        };
        instructions.push(instruction);
    }
    instructions
}

// TASK 1
pub fn task_1(content: &str) -> String {
    let mut expedition = Expedition::from_string(content);
    expedition.reorganize1();

    let result = String::from(expedition.get_top_crates());
    result
}

// TASK 2
pub fn task_2(content: &str) -> u32 {
    let mut total = 0;
    let mut expedition = Expedition::from_string(content);
    expedition.reorganize2();

    total += 1;
    total
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn sample_input_task1() {
        let input = fs::read_to_string("sample_input.txt").expect("File doesn't exist!");
        assert_eq!("CMZ", task_1(&input));
    }

//     #[test]
//     fn sample_input_task2() {
//         let input = "\
// 2-4,6-8
// 2-3,4-5
// 5-7,7-9
// 2-8,3-7
// 6-6,4-6
// 2-6,4-8";
//         assert_eq!(12, task_2(input));
//     }

}
