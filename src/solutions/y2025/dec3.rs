use std::{fs, time::Instant};


pub fn solve() {
    let input = fs::read_to_string("input/2025/input3.txt").unwrap();
    let banks:Vec<Vec<usize>> = input.lines()
        .map(|line| line.chars()
        .map(|c|c.to_string().parse().unwrap()).collect())
        .collect();
    let mut timer = Instant::now();

    println!("Part 1: {:?}", part1(banks.clone()));
    println!("Part 1 took {:?}", timer.elapsed());

    timer = Instant::now();
    println!("Part 2: {:?}", part2(banks.clone()));
    println!("Part 2 took {:?}", timer.elapsed());
}

fn part1(banks: Vec<Vec<usize>>) -> usize {
    banks.into_iter()
    .map(|bank| {
        bank.iter().enumerate()
            .map(|(i, start)| {
                if i == bank.len()-1 {
                    return 0;
                }
                let rest = bank.split_at(i+1).1;
                let biggest_second_num = rest.iter().max().unwrap();
                start*10+biggest_second_num
            })
            .max().unwrap()
    }).sum()
}

fn part2(banks: Vec<Vec<usize>>) -> usize {
    banks
        .iter()
        .map(|bank|part2_solve(bank))
        .sum()
}

fn part2_solve(bank: &Vec<usize>) -> usize {
    let mut result = 0;
    let mut take_size = bank.len() - 11;
    let mut unreserved:Vec<&usize> = bank
        .iter()
        .take(take_size)
        .collect();
    let mut i = 0;

    while unreserved.len() > 0 && result <= 99999999999 {
        let (max_idx, max) = unreserved
            .iter()
            .enumerate()
            .rev()
            .max_by_key(|&(_, e)| e)
            .unwrap();
        result *= 10;
        result += *max;
        
        take_size -= max_idx;
        unreserved = bank
            .iter()
            .skip(i + max_idx + 1)
            .take(take_size)
            .collect();
        i += max_idx + 1;
    }
    println!("RES: {}", result);
    result
}