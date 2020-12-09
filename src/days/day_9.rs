use super::Solver;

const N: u32 = 5;

fn is_sum(entries: &[u64], entry: u64) -> bool {
    for i in 0..N {
        for j in (i+1)..N {
            let entry_i = entries[i as usize];
            let entry_j = entries[j as usize];
            let sum = entry_i + entry_j;
            if sum == entry {
                return true;
            }
        }
    }
    false
}

pub mod part_1 {

    use super::*;

    pub struct State {
        entry_index: u32,
        entries: [u64; N as usize],
    }

    impl super::Solver for State {

        type Input = u64;
        type Output = u64;

        fn deserialize(&self, line: &std::string::String) -> Option<u64> {
            line.parse::<u64>().ok()
        }

        fn serialize(&self, output: &u64) -> std::string::String {
            output.to_string()
        }

        fn new() -> Self {
            State {
                entries: [0; N as usize],
                entry_index: 0,
            }
        }

        fn accumulate(&mut self, entry: u64) -> Option <u64> {

            if self.entry_index >= N {
                if !is_sum(&self.entries, entry) {
                    return Some(entry);
                }
            }

            self.entries[(self.entry_index % N) as usize] = entry;
            self.entry_index += 1;

            None
        }

        fn solve(&mut self) -> Option<u64> {
            return None;
        }

    }

}

pub mod part_2 {

    use super::*;

    pub struct State {
        entries: Vec<u64>,
        prefix_sums: Vec<u64>,
        entry_index: u32,
    }

    fn first_non_sum(entries: &Vec<u64>) -> Option<u64> {
        for (entry_index, entry) in entries.iter().enumerate().skip(N as usize) {
            if !is_sum(&entries[(entry_index-N as usize)..(entry_index as usize)], *entry) {
                return Some(*entry);
            }
        }
        None
    }

    fn range_with_difference(d: u64, entries: &Vec<u64>) -> Option<(u32, u32)> {
        let difference = |(i,j)| {entries[i as usize] - entries[j as usize]};
        let mut range:(u32, u32) = (1,0);
        while range.0 > range.1 && (range.0 as usize) < entries.len() {
            let difference_range = difference(range);
            if difference_range == d {
                return Some(range);
            } else if difference_range < d {
                range.0 += 1;
            } else if difference_range > d {
                range.1 += 1;
            }
        }
        None
    }

    impl super::Solver for State {

        type Input = u64;
        type Output = u64;

        fn deserialize(&self, line: &std::string::String) -> Option<u64> {
            line.parse::<u64>().ok()
        }

        fn serialize(&self, output: &u64) -> std::string::String {
            output.to_string()
        }

        fn new() -> Self {
            State {
                entries: Vec::new(),
                prefix_sums: Vec::new(),
                entry_index: 0,
            }
        }

        fn accumulate(&mut self, entry: u64) -> Option <u64> {
            self.entries.push(entry);
            let sum =
                if self.entry_index > 0 {
                    self.prefix_sums[(self.entry_index-1) as usize]
                } else {
                    0
                };
            self.prefix_sums.push(sum + entry);
            self.entry_index += 1;
            None
        }

        fn solve(&mut self) -> Option<u64> {
            let non_sum = first_non_sum(&self.entries)?;
            let prefix_sum_range = range_with_difference(non_sum, &self.prefix_sums)?;
            let range = (prefix_sum_range.0 - 1, prefix_sum_range.1);
            let mut min = self.entries[range.0 as usize];
            let mut max = self.entries[range.0 as usize];
            for entry in &self.entries[(range.1 as usize)..(range.0 as usize)] {
                if *entry < min {
                    min = *entry;
                }
                if *entry > max {
                    max = *entry;
                }
            }
            return Some(min + max);
        }

    }

}
