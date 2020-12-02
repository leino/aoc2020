use super::Solver;
use regex::Regex;

pub struct Range {
    offset: u32,
    length: u32,
}

pub struct Entry {
    range: Range,
    character: char,
    password: std::string::String,
}

fn deserialize_entry(expression: &regex::Regex, line: &std::string::String) -> Option<Entry> {
    let captures = expression.captures(line)?;
    let min = captures.get(1)?.as_str().parse::<u32>().ok()?;
    let max = captures.get(2)?.as_str().parse::<u32>().ok()?;
    let character = captures.get(3)?.as_str().chars().nth(0)?;
    let password = captures.get(4)?.as_str();
    Some(
        Entry {
            range: Range {
                offset: min,
                length: max - min,
            },
            character: character,
            password: password.to_string()
        }
    )
}


pub mod part_1 {

    use super::*;

    pub struct State {
        valid_password_count: u32,
        input_expression: Regex,
    }

    impl super::Solver for State {

        type Input = Entry;
        type Output = u32;

        fn deserialize(&self, line: &std::string::String) -> Option<Entry> {
            deserialize_entry(&self.input_expression, line)
        }

        fn serialize(&self, output: &u32) -> std::string::String {
            output.to_string()
        }

        fn new() -> Self {
            State {
                valid_password_count: 0,
                input_expression: Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap(),
            }
        }

        fn accumulate(&mut self, entry: Entry) {
            let min = entry.range.offset;
            let max = entry.range.offset + entry.range.length;
            let mut count = 0;
            for c in entry.password.chars() {
                if c == entry.character {
                    count += 1;
                    if count > max {
                        return;
                    }
                }
            }
            if count >= min {
                self.valid_password_count += 1;
            }
        }

        fn solve(&mut self) -> Option<u32> {
            return Some(self.valid_password_count);
        }

    }

}

pub mod part_2 {

    use super::*;

    pub struct State {
        valid_password_count: u32,
        input_expression: Regex,
    }

    impl super::Solver for State {

        type Input = Entry;
        type Output = u32;

        fn deserialize(&self, line: &std::string::String) -> Option<Entry> {
            deserialize_entry(&self.input_expression, line)
        }

        fn serialize(&self, output: &u32) -> std::string::String {
            output.to_string()
        }

        fn new() -> Self {
            State {
                valid_password_count: 0,
                input_expression: Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap(),
            }
        }

        fn accumulate(&mut self, entry: Entry) {
            let index_lo = (entry.range.offset - 1) as usize;
            let index_hi = (entry.range.offset + entry.range.length - 1) as usize;
            let characters: Vec<char> = entry.password.chars().collect();
            if index_hi >= characters.len() {
                return;
            }
            let char_lo = characters[index_lo];
            let char_hi = characters[index_hi];
            let valid = (char_lo == entry.character) != (char_hi == entry.character);
            if valid {
                self.valid_password_count += 1;
            }
        }

        fn solve(&mut self) -> Option<u32> {
            return Some(self.valid_password_count);
        }

    }

}
