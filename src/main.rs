mod day1 {
    pub mod day1;
}

mod day2 {
    pub mod day2;
}
mod utils {
    pub mod utils;
}

use day1::day1::{day1part1, day1part2};
use day2::day2::{day2part1, day2part2};
use utils::utils::{benchmark};

fn main() {
    benchmark(day1part1);
    benchmark(day1part2);
    benchmark(day2part1);
    benchmark(day2part2);
}