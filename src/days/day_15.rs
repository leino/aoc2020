use super::FixedLineCountSolver;
use std::collections::HashMap;

fn from_input(lines: &[String]) -> Option<Vec<u32>> {
    if lines.len() != 1 {
        return None;
    }
    let line = &lines[0];
    let mut numbers = Vec::new();
    for part in line.split(",") {
        numbers.push(part.parse::<u32>().ok()?)
    }
    Some(numbers)
}

fn solve_limit(final_index: u32, numbers: &[u32]) -> Option<u32> {
    if numbers.is_empty() {
        return None;
    }
    let mut turn_seen: HashMap<u32, u32> = HashMap::new();
    let mut turn_index = 1;
    let mut last_number = numbers[0];
    for number in &numbers[1..] {
        turn_seen.insert(last_number, turn_index);
        last_number = *number;
        turn_index += 1;
    }
    for turn_index in turn_index..final_index {
        let number = {
            match turn_seen.get(&last_number) {
                None => 0,
                Some(i) => turn_index - i,
            }
        };
        turn_seen.insert(last_number, turn_index);
        last_number = number;
    }
    Some(last_number)
}


pub mod part_1 {

    use super::*;

    pub struct Instance {
        numbers: Vec<u32>,
    }

    impl super::FixedLineCountSolver for Instance {

        fn from_input(lines: &[String]) -> Option<Instance> {
            Some(Instance {numbers: from_input(lines)?})
        }

        fn solve(&mut self) -> Option<u32> {
            solve_limit(2020, &self.numbers)
        }

    }

}

pub mod part_2 {

    use super::*;

    pub struct Instance {
        numbers: Vec<u32>,
    }

    impl super::FixedLineCountSolver for Instance {

        fn from_input(lines: &[String]) -> Option<Instance> {
            Some(Instance {numbers: from_input(lines)?})
        }

        fn solve(&mut self) -> Option<u32> {
            solve_limit(30000000, &self.numbers)
        }

    }

}
