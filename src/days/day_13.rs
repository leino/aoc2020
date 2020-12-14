use super::FixedLineCountSolver;

pub mod part_1 {

    pub struct Instance {
        earliest_departure_timestamp: u32,
        in_service_bus_ids: Vec<u32>,
    }

    impl super::FixedLineCountSolver for Instance {

        fn from_input(lines: &[String]) -> Option<Instance> {
            if lines.len() != 2 {
                return None;
            }
            let mut ids = Vec::new();
            for id in lines[1].split(",") {
                if id == "x" {
                    continue;
                }
                ids.push(id.parse::<u32>().ok()?);
            }
            Some(
                Instance {
                    earliest_departure_timestamp: lines[0].parse::<u32>().ok()?,
                    in_service_bus_ids: ids,
                }
            )
        }

        fn solve(&mut self) -> Option<u32> {
            let waiting_time =
                |id| {
                    let r = self.earliest_departure_timestamp % id;
                    if r == 0 {0} else {id - r}
                };
            let mut minimum_waiting_time = u32::MAX;
            let mut minimum_wait_id = 0;
            for id in &self.in_service_bus_ids {
                let id_waiting_time = waiting_time(id);
                if id_waiting_time < minimum_waiting_time {
                    minimum_waiting_time = id_waiting_time;
                    minimum_wait_id = *id;
                }
            }
            Some(minimum_wait_id * minimum_waiting_time)
        }

    }

}
