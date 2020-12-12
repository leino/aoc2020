use super::Solver;

pub enum Angle {
    Clockwise90,
    CounterClockwise90,
    Flip, // 180
}

enum Heading {
    East,
    North,
    West,
    South,
}

pub enum Instruction {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Turn(Angle),
    Forward(i32),
}

pub mod part_1 {

    use super::*;

    pub struct State {
        heading: Heading,
        position: (i32, i32),
    }

    fn go(instruction: &Instruction, state: &mut State) {
        match instruction {
            Instruction::West(d) => state.position.0 -= d,
            Instruction::East(d) => state.position.0 += d,
            Instruction::North(d) => state.position.1 += d,
            Instruction::South(d) => state.position.1 -= d,
            Instruction::Forward(d) => {
                match state.heading {
                    Heading::East => go(&Instruction::East(*d), state),
                    Heading::North => go(&Instruction::North(*d), state),
                    Heading::West => go(&Instruction::West(*d), state),
                    Heading::South => go(&Instruction::South(*d), state),
                }
            },
            Instruction::Turn(a) => {
                state.heading =
                    match a {
                        Angle::Flip => {
                            match state.heading {
                                Heading::East => Heading::West,
                                Heading::West => Heading::East,
                                Heading::North => Heading::South,
                                Heading::South => Heading::North,
                            }
                        },
                        Angle::Clockwise90 => {
                            match state.heading {
                                Heading::East => Heading::South,
                                Heading::West => Heading::North,
                                Heading::North => Heading::East,
                                Heading::South => Heading::West,
                            }
                        },
                        Angle::CounterClockwise90 => {
                            match state.heading {
                                Heading::East => Heading::North,
                                Heading::West => Heading::South,
                                Heading::North => Heading::West,
                                Heading::South => Heading::East,
                            }
                        }
                    };
            },
        }
    }

    impl super::Solver for State {

        type Input = Instruction;
        type Output = u32;

        fn deserialize(&self, line: &std::string::String) -> Option<Instruction> {
            let value = &line[1..].parse::<i32>().ok()?;
            match line.chars().nth(0)? {
                'N' => {
                    Some(Instruction::North(*value))
                },
                'S' => {
                    Some(Instruction::South(*value))
                },
                'E' => {
                    Some(Instruction::East(*value))
                },
                'W' => {
                    Some(Instruction::West(*value))
                },
                'L' => {
                    let angle =
                        match value {
                            90 => Some(Angle::CounterClockwise90),
                            270 => Some(Angle::Clockwise90),
                            180 => Some(Angle::Flip),
                            _ => None,
                        };
                    Some(Instruction::Turn(angle?))
                },
                'R' => {
                    let angle =
                        match value {
                            90 => Some(Angle::Clockwise90),
                            270 => Some(Angle::CounterClockwise90),
                            180 => Some(Angle::Flip),
                            _ => None,
                        };
                    Some(Instruction::Turn(angle?))
                },
                'F' => {
                    Some(Instruction::Forward(*value))
                },
                _ => {
                    None
                },
            }
        }

        fn serialize(&self, output: &u32) -> std::string::String {
            output.to_string()
        }

        fn new() -> Self {
            State {
                position: (0, 0),
                heading: Heading::East,
            }
        }

        fn accumulate(&mut self, instruction: Instruction) -> Option<u32> {
            go(&instruction, self);
            None
        }

        fn solve(&mut self) -> Option<u32> {
            Some((self.position.0.abs() + self.position.1.abs()) as u32)
        }

    }

}
