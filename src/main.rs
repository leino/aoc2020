use std::fmt;
use std::io::{self, BufRead};
use std::io::Write;
mod days;
use days::Solver;

struct Parameters {
    day_index: u32,
    part_index: u32,
    input_file_path: std::path::PathBuf,
    output_file_path: std::path::PathBuf,
}

struct Runner<'a> {
    input_file_path: &'a std::path::Path,
    output_file_path: &'a std::path::Path,
    input_file: &'a std::fs::File,
    output_file: &'a mut std::fs::File,
}

pub trait Runnable {
    fn run<S: Solver>(&mut self, solver: &mut S);
}

impl Runnable for Runner<'_> {
    fn run<S: Solver>(&mut self, solver: &mut S) {
        let mut output = None;
        for (line_index, line_result) in io::BufReader::new(self.input_file).lines().enumerate() {
            match line_result {
                Err(re) => {
                    println!("Read error for file {}: {}", self.input_file_path.display(), re);
                    std::process::exit(1);
                },
                Ok(line) => {
                    match solver.deserialize(&line) {
                        None => {
                            println!("{}:{}: Failed to parse line.",
                                     self.input_file_path.display(),
                                     line_index);
                            std::process::exit(1);
                        },
                        Some(input) => {
                            output = solver.accumulate(input);
                            if output.is_some() {
                                break;
                            }
                        },
                    }
                },
            };
        }
        match output.or(solver.solve()) {
            None => {
                println!("Failed to solve: invalid input.");
                std::process::exit(1);
            },
            Some(output) => {
                match self.output_file.write_fmt(format_args!("{}\n", solver.serialize(&output))) {
                    Ok(()) => (),
                    Err(we) => {
                        println!("Write error for file {}: {}",
                                 self.output_file_path.display(), we);
                        std::process::exit(1);
                    },
                }
            }
        }
    }
}

enum ReadArgumentsError {
    NoDayIndexGiven,
    DayIndexInvalid(String, std::num::ParseIntError),
    NoPartIndexGiven,
    PartIndexInvalid(String, std::num::ParseIntError),
    NoInputFilePathGiven,
    NoOutputFilePathGiven
}

enum ParametersError {
    DayIndexOutOfBounds(u32)
}

impl fmt::Display for ReadArgumentsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReadArgumentsError::NoDayIndexGiven => write!(f, "No day index given."),
            ReadArgumentsError::DayIndexInvalid(s, e) => write!(f, "Day index invalid: '{}': {}.", s, e),
            ReadArgumentsError::NoPartIndexGiven => write!(f, "No part index given."),
            ReadArgumentsError::PartIndexInvalid(s, e) => write!(f, "Part index invalid: '{}': {}.", s, e),
            ReadArgumentsError::NoInputFilePathGiven => write!(f, "No input file path given."),
            ReadArgumentsError::NoOutputFilePathGiven => write!(f, "No output file path given."),
        }
    }
}

impl fmt::Display for ParametersError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParametersError::DayIndexOutOfBounds(i) =>
                write!(f, "Day index out of bounds: '{}': should be between 1 and 24.", i),
        }
    }
}


fn read_arguments() -> Result<Parameters, ReadArgumentsError> {
    let day_index: u32 =
        match std::env::args().nth(1) {
            None => return Err(ReadArgumentsError::NoDayIndexGiven),
            Some(s) => {
                match s.parse() {
                    Err(e) => return Err(ReadArgumentsError::DayIndexInvalid(s.clone(), e)),
                    Ok(i) => i
                }
            }
        };
    let part_index: u32 =
        match std::env::args().nth(2) {
            None => return Err(ReadArgumentsError::NoPartIndexGiven),
            Some(s) => {
                match s.parse() {
                    Err(e) => return Err(ReadArgumentsError::PartIndexInvalid(s.clone(), e)),
                    Ok(i) => i
                }
            }
        };
    let input_file_path =
        match std::env::args().nth(3) {
            None => return Err(ReadArgumentsError::NoInputFilePathGiven),
            Some(s) => std::path::PathBuf::from(s)
        };
    let output_file_path =
        match std::env::args().nth(4) {
            None => return Err(ReadArgumentsError::NoOutputFilePathGiven),
            Some(s) => std::path::PathBuf::from(s)
        };
    Ok(
        Parameters {
            day_index: day_index,
            part_index: part_index,
            input_file_path: input_file_path,
            output_file_path: output_file_path
        }
    )
}

impl Parameters {
    fn validate(&self) -> Option<ParametersError> {
        if self.day_index < 1 || self.day_index > 24 {
            return Some(ParametersError::DayIndexOutOfBounds(self.day_index));
        }
        None
    }
}

fn main() {
    let parameters =
        match read_arguments() {
            Err(e) => {
                println!("Invalid arguments: {}", e);
                std::process::exit(1);
            },
            Ok(p) =>
                match p.validate() {
                    Some(e) => {
                        println!("Invalid parameters: {}", e);
                        std::process::exit(1);
                    },
                    None => p
                }
        };
    let input_file = {
        let path = parameters.input_file_path.as_path();
        match std::fs::File::open(path) {
            Err(e) => {
                println!("Failed to open input file '{}': {}.", path.display(), e);
                std::process::exit(1);
            },
            Ok(f) => f
        }
    };
    let mut output_file = {
        let file_path = parameters.output_file_path.as_path();
        match file_path.parent() {
            None => (),
            Some(directory_path) => {
                if !std::path::Path::is_dir(directory_path) {
                    match std::fs::create_dir_all(directory_path) {
                        Err(cde) => {
                            println!("The directory '{}' does not exist, and could not be created: {}.",
                                     directory_path.display(), cde);
                            std::process::exit(1);
                        },
                        Ok(()) => (),
                    }
                }
            }
        };

        match std::fs::File::create(file_path) {
            Err(ce) => {
                println!("Failed to open output file '{}': {}.", file_path.display(), ce);
                std::process::exit(1);
            },
            Ok(f) => f
        }
    };

    let mut runner =
        Runner {
            input_file: &input_file,
            input_file_path: parameters.input_file_path.as_path(),
            output_file: &mut output_file,
            output_file_path: parameters.output_file_path.as_path()
        };
    match (parameters.day_index, parameters.part_index) {
        (1, 1) => runner.run(&mut days::day_1::part_1::State::new()),
        (1, 2) => runner.run(&mut days::day_1::part_2::State::new()),
        (2, 1) => runner.run(&mut days::day_2::part_1::State::new()),
        (2, 2) => runner.run(&mut days::day_2::part_2::State::new()),
        (3, 1) => runner.run(&mut days::day_3::part_1::State::new()),
        (3, 2) => runner.run(&mut days::day_3::part_2::State::new()),
        (6, 1) => runner.run(&mut days::day_6::part_1::State::new()),
        (6, 2) => runner.run(&mut days::day_6::part_2::State::new()),
        (8, 1) => runner.run(&mut days::day_8::part_1::State::new()),
        (8, 2) => runner.run(&mut days::day_8::part_2::State::new()),
        (12, 1) => runner.run(&mut days::day_12::part_1::State::new()),
        _ => {
            println!("Solver not implemented for day {} part {}.",
                     parameters.day_index, parameters.part_index);
            std::process::exit(1);
        },
    }

}
