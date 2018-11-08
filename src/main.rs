extern crate advent_of_code_2015;
extern crate aoc_runner;

fn main() {
    use advent_of_code_2015::*;
    use aoc_runner::ArcStr;

    println!(
        "DAY5 - PART1: {}",
        Factory::day5_part1(ArcStr::from(include_str!("../input/day5"))).run()
    );
    println!(
        "DAY5 - PART2: {}",
        Factory::day5_part2(ArcStr::from(include_str!("../input/day5"))).run()
    );
}
