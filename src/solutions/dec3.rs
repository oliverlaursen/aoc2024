use regex::Regex;
use std::fs;

pub fn solve() {
    let input = fs::read_to_string("input/input3.txt").unwrap();
    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
}

fn part1(input: &String) -> i64 {
    let re = Regex::new(r"mul\([0-9]*,[0-9]*\)").unwrap();
    re.find_iter(&input)
        .map(|expr| expr.as_str().to_string())
        .map(calc_mul_expr)
        .sum()
}

fn part2(input: &String) -> i64 {
    let re = Regex::new(r"mul\([0-9]*,[0-9]*\)|do\(\)|don't\(\)").unwrap();
    let mut enabled = true;
    re.find_iter(&input)
        .map(|expr| expr.as_str().to_string())
        .map(|expr| match expr.as_str() {
            "do()" => {
                enabled = true;
                return 0;
            }
            "don't()" => {
                enabled = false;
                return 0;
            }
            _ => {
                if !enabled {
                    return 0;
                }
                calc_mul_expr(expr)
            }
        })
        .sum()
}

fn calc_mul_expr(expr: String) -> i64 {
    let mut pair = expr.split(',').into_iter();
    let first = pair.next().unwrap().trim_start_matches("mul(");
    let second = pair.next().unwrap().trim_end_matches(")");
    first.parse::<i64>().unwrap() * second.parse::<i64>().unwrap()
}
