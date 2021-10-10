use crate::prelude::*;
use log::info;

pub fn log_result(year: u32, day: u8, answers: (PartAnswer, PartAnswer)) {
    let (part_one, part_two) = answers;

    info!(
        "year {}, day {}, part 1: {} ({:?} ms)",
        year,
        day,
        part_one.get_answer(),
        part_one.get_duration().as_millis()
    );
    info!(
        "year {}, day {}, part 2: {} ({:?} ms)",
        year,
        day,
        part_two.get_answer(),
        part_two.get_duration().as_millis()
    );
}
