#[macro_use]
extern crate criterion;
extern crate advent_of_code_2015 as aoc;
extern crate aoc_runner;

use aoc::day1;
use aoc_runner::*;
use criterion::{Criterion, Fun};

fn criterion_benchmark(c: &mut Criterion) {
    let runner = Generator::simple(1, day1::part1).generate();
    let bytes = Fun::new("Bytes", move |b, _| b.iter(|| runner.run()));

    let runner = Generator::simple(1, day1::part1_char).generate();
    let chars = Fun::new("Chars", move |b, _| b.iter(|| runner.run()));
    c.bench_functions("part1", vec![bytes, chars], ());

    let runner = Generator::simple(1, day1::part2).generate();
    let bytes = Fun::new("Bytes", move |b, _| b.iter(|| runner.run()));
    let runner = Generator::simple(1, day1::part2_char).generate();
    let chars = Fun::new("Chars", move |b, _| b.iter(|| runner.run()));
    c.bench_functions("part2", vec![bytes, chars], ());
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);