use super::Solver;

type Questions = u32;

pub enum Row {
    GroupTerminator,
    Response(Questions),
}

const ALL_QUESTIONS: Questions = 0x03ffffff;

pub mod part_1 {

    use super::*;

    pub struct State {
        questions: u32,
        yes_count: u32
    }

    impl super::Solver for State {

        type Input = Row;
        type Output = u32;

        fn deserialize(&self, line: &std::string::String) -> Option<Row> {

            if line.is_empty() {
                return Some(Row::GroupTerminator);
            }

            let mut questions: Questions = 0;
            for c in line.chars() {
                if c < 'a' || c > 'z' {
                    return None
                }
                let question_index = (c as u32) - ('a' as u32);
                questions |= 1 << question_index;
            }

            return Some(Row::Response(questions))

        }

        fn serialize(&self, output: &u32) -> std::string::String {
            output.to_string()
        }

        fn new() -> Self {
            State {
                questions: 0,
                yes_count: 0,
            }
        }

        fn accumulate(&mut self, row: Row) {
            match row {
                Row::GroupTerminator => {
                    self.yes_count += self.questions.count_ones();
                    self.questions = 0;
                },
                Row::Response(questions) => {
                    self.questions |= questions;
                },
            }
        }

        fn solve(&mut self) -> Option<u32> {
            return Some(self.yes_count + self.questions.count_ones());
        }

    }

}


pub mod part_2 {

    use super::*;

    pub struct State {
        yes_questions: Questions,
        yes_count: u32
    }

    impl super::Solver for State {

        type Input = Row;
        type Output = u32;

        fn deserialize(&self, line: &std::string::String) -> Option<Row> {

            if line.is_empty() {
                return Some(Row::GroupTerminator);
            }

            let mut questions: Questions = 0;
            for c in line.chars() {
                if c < 'a' || c > 'z' {
                    return None
                }
                let question_index = (c as u32) - ('a' as u32);
                questions |= 1 << question_index;
            }

            return Some(Row::Response(questions))

        }

        fn serialize(&self, output: &u32) -> std::string::String {
            output.to_string()
        }

        fn new() -> Self {
            State {
                yes_questions: ALL_QUESTIONS,
                yes_count: 0,
            }
        }

        fn accumulate(&mut self, row: Row) {
            match row {
                Row::GroupTerminator => {
                    self.yes_count += self.yes_questions.count_ones();
                    self.yes_questions = ALL_QUESTIONS;
                },
                Row::Response(questions) => {
                    self.yes_questions &= questions;
                },
            }
        }

        fn solve(&mut self) -> Option<u32> {
            return Some(self.yes_count + self.yes_questions.count_ones());
        }

    }

}
