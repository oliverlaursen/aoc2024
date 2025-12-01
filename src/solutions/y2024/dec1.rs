use std::fs;

pub fn solve() {
    let input = fs::read_to_string("input/input1.txt").unwrap();
    let (mut left_list, mut right_list): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|line| {
            let mut nums = line.split_whitespace().map(|n| n.parse::<u32>().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .unzip();
    println!("Part 1: {}", part1(&mut left_list, &mut right_list));
    println!("Part 2: {}", part2(&left_list, &right_list));
}

pub fn part2(left: &Vec<u32>, right: &Vec<u32>) -> u32 {
    left.iter()
        .map(|id| right.iter().filter(|&x| x == id).count() as u32 * id)
        .sum()
}

pub fn part1(left: &mut Vec<u32>, right: &mut Vec<u32>) -> u32 {
    left.sort();
    right.sort();
    left.into_iter().zip(right)
        .map(|x|x.0.abs_diff(*x.1))
        .sum()
}

