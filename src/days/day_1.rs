use super::Solver;

pub mod part_1 {

    pub struct State {
        entries: std::vec::Vec<u32>,
    }

    impl super::Solver for State {

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
