// use std::error::Error;

struct Elf {
    section_start: u32,
    section_end: u32,
}

impl Elf {
    fn from_vec(sections: &Vec<&str>) -> Elf {
        Elf {
            section_start: sections[0].parse::<u32>().unwrap(),
            section_end: sections[1].parse::<u32>().unwrap(),
        }
    }
    fn overlap_fully(&self, elf: &Elf) -> bool {
        if self.section_start <= elf.section_start && self.section_end >= elf.section_end ||
            elf.section_start <= self.section_start && elf.section_end >= self.section_end {
            return true
        }
        false
    }
    fn overlap_partial(&self, elf: &Elf) -> bool {
        if self.section_start > elf.section_end || self.section_end < elf.section_start {
            return false
        }
        true
    }
}

// TASK 1
pub fn task_1(content: &str) -> u32 {
    let mut total = 0;
    for line in content.lines() {
        let pair: Vec<&str> = line.split(',').collect();
        let elf1 = Elf::from_vec(&pair[0].split('-').collect());
        let elf2 = Elf::from_vec(&pair[1].split('-').collect());
        if elf1.overlap_fully(&elf2) {
            total += 1;
        }
    }
    total
}

// TASK 2
pub fn task_2(content: &str) -> u32 {
    let mut total = 0;
    for line in content.lines() {
        let pair: Vec<&str> = line.split(',').collect();
        let elf1 = Elf::from_vec(&pair[0].split('-').collect());
        let elf2 = Elf::from_vec(&pair[1].split('-').collect());
        if elf1.overlap_partial(&elf2) {
            total += 1;
        }
    }
    total
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_task1() {
        let input = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        assert_eq!(2, task_1(input));
    }

    #[test]
    fn sample_input_task2() {
        let input = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        assert_eq!(4, task_2(input));
    }

}
