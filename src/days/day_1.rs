use super::Solver;

pub mod part_1 {

    pub struct State {
        entries: std::vec::Vec<u32>,
    }

    impl super::Solver for State {

        type Input = u32;
        type Output = u32;

        fn deserialize(&self, line: &std::string::String) -> Option<u32> {
            line.parse::<u32>().ok()
        }

        fn serialize(&self, output: &u32) -> std::string::String {
            output.to_string()
        }

        fn new() -> Self {
            State {
                entries: std::vec::Vec::new(),
            }
        }

        fn accumulate(&mut self, entry: u32) {
            self.entries.push(entry);
        }

        fn solve(&mut self) -> Option<u32> {
            for a in &self.entries {
                for b in &self.entries {
                    if a + b == 2020 {
                        return Some(a * b);
                    }
                }
            }
            return None;
        }

    }

}

pub mod part_2 {

    pub struct State {
        entries: std::vec::Vec<u32>,
    }

    impl super::Solver for State {

        type Input = u32;
        type Output = u32;

        fn deserialize(&self, line: &std::string::String) -> Option<u32> {
            line.parse::<u32>().ok()
        }

        fn serialize(&self, output: &u32) -> std::string::String {
            output.to_string()
        }

        fn new() -> Self {
            State {
                entries: std::vec::Vec::new(),
            }
        }

        fn accumulate(&mut self, entry: u32) {
            self.entries.push(entry);
        }

        fn solve(&mut self) -> Option<u32> {
            for a in &self.entries {
                for b in &self.entries {
                    for c in &self.entries {
                        if a + b + c  == 2020 {
                            return Some(a * b * c);
                        }
                    }
                }
            }
            return None;
        }

    }

}
