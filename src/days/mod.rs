pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_6;
pub mod day_8;

pub trait Solver {
    type Input;
    type Output;
    fn deserialize(&self, line: &std::string::String) -> Option<Self::Input>;
    fn serialize(&self, output: &Self::Output) -> std::string::String;
    fn new() -> Self;
    fn accumulate(&mut self, x: Self::Input);
    fn solve(&mut self) -> Option<Self::Output>;
}
