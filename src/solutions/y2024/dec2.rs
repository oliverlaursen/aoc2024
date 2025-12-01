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
    let increasing = report.is_sorted();
    let decreasing = report.is_sorted_by(|a, b| a>b);
    report.windows(2).map(|w| {
        let left = w[0];
        let right = w[1];
        left.abs_diff(right)
        }).all(|diff| diff >= 1 && diff <= 3 && (increasing || decreasing))
}
