pub trait Solver {
    fn new() -> Self;
    fn accumulate(&mut self, x: u32);
    fn solve(&mut self) -> Option<u32>;
}
