use super::Solver;

// All rows of the map are 31 bits long
const ROW_LENGTH: u32 = 31;
type Row = u32;
const SLOPE_COUNT: usize = 5;
const SLOPES: [(u32, u32); SLOPE_COUNT] = [(1, 1),
                                           (3, 1),
                                           (5, 1),
                                           (7, 1),
                                           (1, 2)];

fn deserialize_row(line: &std::string::String) -> Option<Row> {

    if line.len() != (ROW_LENGTH as usize) {
        return None;
    }

    let mut row: u32 = 0;
    for c in line.chars() {
        match c {
            '.' => {
                row = (row << 1) + 0;
            },
            '#' => {
                row = (row << 1) + 1;
            },
            _ => {
                return None
            }
        }
    }

    Some(row)
}


pub mod part_1 {

    use super::*;

    pub struct State {
        tree_hit_count: u32,
        row_index: u32,
    }

    impl super::Solver for State {

        type Input = Row;
        type Output = u32;

        fn deserialize(&self, line: &std::string::String) -> Option<Row> {
            deserialize_row(line)
        }

        fn serialize(&self, output: &u32) -> std::string::String {
            output.to_string()
        }

        fn new() -> Self {
            State {
                tree_hit_count: 0,
                row_index: 0,
            }
        }

        fn accumulate(&mut self, row: Row) {
            let col_index = (3*self.row_index) % ROW_LENGTH;
            let col_mask = 1 << (ROW_LENGTH - 1 - col_index);
            let hit_tree = (col_mask & row) != 0;
            if hit_tree {
                self.tree_hit_count += 1;
            }
            self.row_index += 1;
        }

        fn solve(&mut self) -> Option<u32> {
            return Some(self.tree_hit_count);
        }

    }

}

pub mod part_2 {

    use super::*;

    pub struct State {
        tree_hit_count: [u32; SLOPE_COUNT],
        row_index: u32,
    }

    impl super::Solver for State {

        type Input = Row;
        type Output = u32;

        fn deserialize(&self, line: &std::string::String) -> Option<Row> {
            deserialize_row(line)
        }

        fn serialize(&self, output: &u32) -> std::string::String {
            output.to_string()
        }

        fn new() -> Self {
            State {
                tree_hit_count: [0; SLOPE_COUNT],
                row_index: 0,
            }
        }

        fn accumulate(&mut self, row: Row) {
            for (slope_index, slope) in SLOPES.iter().enumerate() {
                let col_index = (slope.0*(self.row_index/slope.1)) % ROW_LENGTH;
                let col_mask: u32 = {
                    if (self.row_index % slope.1) == 0 {
                        1 << (ROW_LENGTH - 1 - col_index)
                    } else {
                        0
                    }
                };
                let hit = (col_mask & row) != 0;
                if hit {
                    self.tree_hit_count[slope_index] += 1;
                }
            }
            self.row_index += 1;
        }

        fn solve(&mut self) -> Option<u32> {
            let mut product = 1;
            for count in self.tree_hit_count.iter() {
                product *= count;
            }
            return Some(product);
        }

    }

}
