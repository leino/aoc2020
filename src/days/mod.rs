pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_6;
pub mod day_8;
pub mod day_12;
pub mod day_13;

pub trait CumulativeSolver {
    type Input;
    type Output;
    fn deserialize(&self, line: &std::string::String) -> Option<Self::Input>;
    fn serialize(&self, output: &Self::Output) -> std::string::String;
    fn new() -> Self;
    // Returns some output if it's done early.
    fn accumulate(&mut self, x: Self::Input) -> Option<Self::Output>;
    fn solve(&mut self) -> Option<Self::Output>;
}

pub trait FixedLineCountSolver {
    fn from_input(lines: &[String]) -> Option<Self> where Self: Sized;
    fn solve(&mut self) -> Option<u32>;
}
