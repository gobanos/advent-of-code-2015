extern crate advent_of_code_2015;
extern crate aoc_runner;

fn main() {
    use advent_of_code_2015::*;
    use aoc_runner::ArcStr;

    println!(
        "DAY4 - PART1: {}",
        Factory::day4_part1(ArcStr::from(include_str!("../input/day4"))).run()
    );
    println!(
        "DAY4 - PART2: {}",
        Factory::day4_part2(ArcStr::from(include_str!("../input/day4"))).run()
    );
}
