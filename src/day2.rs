type Gift = (u32, u32, u32);

aoc! {
    #[generator]
    pub fn input_generator(input: &str) -> Vec<Gift> {
        input
            .lines()
            .map(|l| {
                let mut gift = l.trim().split('x').map(|d| d.parse().unwrap());
                (
                    gift.next().unwrap(),
                    gift.next().unwrap(),
                    gift.next().unwrap(),
                )
            }).collect()
    }

    #[aoc(day2, part1)]
    pub fn solve_part1(input: &[Gift]) -> u32 {
        input
            .iter()
            .map(|&(l, w, h)| {
                let (s1, s2) = smallest_side((l, w, h));

                2 * l * w + 2 * w * h + 2 * h * l + s1 * s2
            }).sum()
    }

    #[aoc(day2, part2)]
    pub fn solve_part2(input: &[Gift]) -> u32 {
        input
            .iter()
            .map(|&(l, w, h)| {
                let (s1, s2) = smallest_side((l, w, h));

                (s1 + s2) * 2 + l * w * h
            }).sum()
    }
}

//pub fn input_generator(input: &str) -> Vec<Gift> {
//    input
//        .lines()
//        .map(|l| {
//            let mut gift = l.trim().split('x').map(|d| d.parse().unwrap());
//            (
//                gift.next().unwrap(),
//                gift.next().unwrap(),
//                gift.next().unwrap(),
//            )
//        }).collect()
//}
//
//pub use self::solve_part1::runner as solve_part1;
//mod solve_part1 {
//    use aoc_runner::{Runner, ArcStr};
//    use super::*;
//
//    pub fn runner(input: &[Gift]) -> u32 {
//        input
//            .iter()
//            .map(|&(l, w, h)| {
//                let (s1, s2) = smallest_side((l, w, h));
//
//                2 * l * w + 2 * w * h + 2 * h * l + s1 * s2
//            }).sum()
//    }
//
//    #[derive(Runner)]
//    #[runner(runner, u32, input_generator)]
//    pub struct RunnerStruct {
//        input: Vec<Gift>,
//    }
//}
//
//pub use self::solve_part2::runner as solve_part2;
//mod solve_part2 {
//    use aoc_runner::{Runner, ArcStr};
//    use super::*;
//
//    pub fn runner(input: &[Gift]) -> u32 {
//        input
//            .iter()
//            .map(|&(l, w, h)| {
//                let (s1, s2) = smallest_side((l, w, h));
//
//                (s1 + s2) * 2 + l * w * h
//            }).sum()
//    }
//
//    #[derive(Runner)]
//    #[runner(runner, u32, input_generator)]
//    pub struct RunnerStruct {
//        input: Vec<Gift>,
//    }
//}

fn smallest_side((l, w, h): Gift) -> (u32, u32) {
    let mut vec = vec![l, w, h];
    vec.sort();

    (vec[0], vec[1])
}

//// AUTO GEN
//#[derive(Runner)]
//#[runner(solve_part1, u32, input_generator)]
//pub struct Day2Part1 {
//    input: Vec<Gift>,
//}
//
//#[derive(Runner)]
//#[runner(solve_part2, u32, input_generator)]
//pub struct Day2Part2 {
//    input: Vec<Gift>,
//}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // A present with dimensions 2x3x4 requires 2*6 + 2*12 + 2*8 = 52 square feet of wrapping paper plus 6 square feet of slack, for a total of 58 square feet.
    fn example1() {
        assert_eq!(Day2Part1::gen(ArcStr::from("2x3x4")).run(), 58);
    }

    #[test]
    // A present with dimensions 1x1x10 requires 2*1 + 2*10 + 2*10 = 42 square feet of wrapping paper plus 1 square foot of slack, for a total of 43 square feet.
    fn example2() {
        assert_eq!(Day2Part1::gen(ArcStr::from("1x1x10")).run(), 43);
    }

    #[test]
    // A present with dimensions 2x3x4 requires 2+2+3+3 = 10 feet of ribbon to wrap the present plus 2*3*4 = 24 feet of ribbon for the bow, for a total of 34 feet.
    fn example3() {
        assert_eq!(Day2Part2::gen(ArcStr::from("2x3x4")).run(), 34);
    }

    #[test]
    // A present with dimensions 1x1x10 requires 1+1+1+1 = 4 feet of ribbon to wrap the present plus 1*1*10 = 10 feet of ribbon for the bow, for a total of 14 feet.
    fn example4() {
        assert_eq!(Day2Part2::gen(ArcStr::from("1x1x10")).run(), 14);
    }

}
