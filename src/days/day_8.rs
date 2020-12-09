use super::Solver;
use std::collections::HashSet;

#[derive(Copy, Clone, Debug)]
pub enum Instruction {
    Accumulate(i32),
    Jump(i32),
    NoOperation(i32),
}

#[derive(PartialEq)]
enum Outcome {
    InfiniteLoop,
    InvalidJump,
    Termination
}

#[derive(Clone)]
struct MachineState {
    accumulator: i32,
    instruction_index: u32,
    executed_instructions: HashSet<u32>,
}

fn step<F: Fn(u32) -> Instruction>(instruction: F, instructions_length: u32, state: &mut MachineState) -> Option<Outcome> {
    if state.instruction_index == instructions_length {
        return Some(Outcome::Termination);
    }
    if state.executed_instructions.contains(&state.instruction_index) {
        return Some(Outcome::InfiniteLoop);
    }
    state.executed_instructions.insert(state.instruction_index);
    match instruction(state.instruction_index) {
        Instruction::Accumulate(x) => {
            state.accumulator += x;
            state.instruction_index += 1;
        },
        Instruction::Jump(x) => {
            let destination_invalid =
                (x < 0 && state.instruction_index < -x as u32) ||
                (((instructions_length - state.instruction_index) as i32) < x);
            if destination_invalid {
                return Some(Outcome::InvalidJump);
            } else {
                state.instruction_index = state.instruction_index.wrapping_add(x as u32);
            }
        },
        Instruction::NoOperation(_) => {
            state.instruction_index += 1;
        },
    }
    None
}

fn run(instructions: &Vec<Instruction>) -> (Outcome, i32) {
    let mut state = MachineState {
        accumulator: 0,
        instruction_index: 0,
        executed_instructions: HashSet::<u32>::new(),
    };
    loop {
        match step(|i| {instructions[i as usize]}, instructions.len() as u32, &mut state) {
            Some(o) => return (o, state.accumulator),
            _ => continue,
        }
    }
}

fn deserialize_instruction(line: &std::string::String) -> Option<Instruction> {
    let instruction_parts: Vec<&str> = line.split(" ").collect();
    if instruction_parts.len() != 2 {
        return None;
    }
    let argument = instruction_parts[1].parse::<i32>().ok()?;
    let instruction =
        match instruction_parts[0] {
            "acc" => Instruction::Accumulate(argument),
            "jmp" => Instruction::Jump(argument),
            "nop" => Instruction::NoOperation(argument),
            _ => return None,
        };
    Some(instruction)
}


pub mod part_1 {

    use super::*;

    pub struct State {
        instructions: Vec<Instruction>,
    }

    impl super::Solver for State {

        type Input = Instruction;
        type Output = i32;

        fn deserialize(&self, line: &std::string::String) -> Option<Instruction> {
            deserialize_instruction(line)
        }

        fn serialize(&self, output: &i32) -> std::string::String {
            output.to_string()
        }

        fn new() -> Self {
            State {
                instructions: Vec::new(),
            }
        }

        fn accumulate(&mut self, instruction: Instruction) -> Option<i32> {
            self.instructions.push(instruction);
            None
        }

        fn solve(&mut self) -> Option<i32> {
            match run(&self.instructions) {
                (Outcome::InfiniteLoop, a) => Some(a),
                _ => None
            }
        }

    }

}

pub mod part_2 {

    use super::*;

    pub struct State {
        instructions: Vec<Instruction>,
    }

    fn corrupted(inst: &Instruction) -> Instruction {
        match inst {
            Instruction::Jump(x) => Instruction::NoOperation(*x),
            Instruction::NoOperation(x) => Instruction::Jump(*x),
            _ => *inst,
        }
    }

    fn possibly_corrupt(inst: &Instruction) -> bool {
        match inst {
            Instruction::Jump(_) => true,
            Instruction::NoOperation(_) => true,
            _ => false,
        }
    }

    impl super::Solver for State {

        type Input = Instruction;
        type Output = i32;

        fn deserialize(&self, line: &std::string::String) -> Option<Instruction> {
            deserialize_instruction(line)
        }

        fn serialize(&self, output: &i32) -> std::string::String {
            output.to_string()
        }

        fn new() -> Self {
            State {
                instructions: Vec::new(),
            }
        }

        fn accumulate(&mut self, instruction: Instruction) -> Option<i32> {
            self.instructions.push(instruction);
            None
        }

        fn solve(&mut self) -> Option<i32> {
            let mut state = MachineState {
                accumulator: 0,
                instruction_index: 0,
                executed_instructions: HashSet::<u32>::new(),
            };

            loop {
                let instruction = &self.instructions[state.instruction_index as usize];

                // Go on an excursion if it's possible to corrupt the instruction.
                if possibly_corrupt(instruction) {
                    let corrupted_instruction_index = Some(state.instruction_index);
                    let current_instruction = |instruction_index| {
                        let uncorrupted = &self.instructions[instruction_index as usize];
                        if corrupted_instruction_index == Some(instruction_index) {
                            corrupted(uncorrupted)
                        } else {
                            *uncorrupted
                        }
                    };

                    let mut excursion_state = state.clone();
                    let excursion_outcome =
                        loop {
                            let step_outcome = step(current_instruction,
                                                    self.instructions.len() as u32, &mut excursion_state);
                            match step_outcome {
                                Some(o) => break o,
                                _ => continue,
                            }
                        };
                    if excursion_outcome == Outcome::Termination {
                        return Some(excursion_state.accumulator);
                    }
                }

                // The excursion yielded nothing, so continue as normal.
                match step(|_| {*instruction},
                           self.instructions.len() as u32, &mut state) {
                    Some(_) => break None,
                    None => continue,
                }
            }

        }

    }

}
