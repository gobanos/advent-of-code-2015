use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
enum Action {
    TurnOn(Rectangle),
    TurnOff(Rectangle),
    Toggle(Rectangle),
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        unimplemented!()
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Rectangle {
    a: Point,
    b: Point,
}

#[derive(Debug, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

struct Grid([[bool; 1000]; 1000]);

#[aoc_generator(day6)]
fn parse_actions(input: &str) -> Vec<Action> {
    input.lines().map(|l| l.trim().parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser() {
        assert_eq!(
            Ok(Action::TurnOn(Rectangle {
                a: Point { x: 0, y: 0 },
                b: Point { x: 999, y: 999 }
            })),
            "turn on 0,0 through 999,999".parse()
        );
    }
}
