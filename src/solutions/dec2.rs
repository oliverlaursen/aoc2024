use std::fs;

pub fn solve() {
    let input = fs::read_to_string("input/input2.txt").unwrap();
    let input: Vec<Vec<i64>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect()
        })
        .collect();
    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
}

fn part1(input: &Vec<Vec<i64>>) -> usize {
    input.into_iter().cloned().filter(report_is_valid).count()
}

fn part2(input: &Vec<Vec<i64>>) -> usize {
    input
        .into_iter()
        .cloned()
        .filter(|x| get_variants(x).iter().any(report_is_valid))
        .count()
}

fn get_variants(report: &Vec<i64>) -> Vec<Vec<i64>> {
    let mut variants = Vec::new();
    for i in 0..report.len() {
        let mut variation = report.clone();
        variation.remove(i);
        variants.push(variation);
    }
    variants
}

fn report_is_valid(report: &Vec<i64>) -> bool {
    let first = report.first().unwrap();
    let second = report.get(1).unwrap();
    let increasing = (first - second) < 0;

    for slice in report.windows(2) {
        let left = slice.first().unwrap();
        let right = slice.last().unwrap();
        let diff = left.abs_diff(*right);
        let inc = (left - right) < 0;
        if diff < 1 || diff > 3 || increasing != inc {
            return false;
        }
    }
    true
}
