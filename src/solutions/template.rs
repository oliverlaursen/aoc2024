use std::{fs, time::Instant};

pub fn solve() {
    let input = fs::read_to_string("input/input3.txt").unwrap();
    let mut timer = Instant::now();

    println!("Part 1: {:?}", part1(&input));
    println!("Part 1 took {:?}", timer.elapsed());

    timer = Instant::now();
    println!("Part 2: {:?}", part2(&input));
    println!("Part 2 took {:?}", timer.elapsed());
}

fn part1(input: &String) -> i64 {
    todo!()
}

fn part2(input: &String) -> i64 {
    todo!()
}