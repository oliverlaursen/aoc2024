use std::{collections::LinkedList, fs, time::Instant};

pub fn solve() {
    let input = fs::read_to_string("input/input9.txt").unwrap();
    let mut timer = Instant::now();
    let digits: Vec<i64> = input
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect();
    let mut l: Vec<Option<i64>> = Vec::new();
    let mut stack: LinkedList<i64> = LinkedList::new();
    let mut counter = 0;
    digits.iter().enumerate().for_each(|(i, d)| {
        let file = i % 2 == 0;
        if file {
            for _ in 0..*d {
                l.push(Some(counter));
                stack.push_back(counter);
            }
            counter += 1;
        } else {
            for _ in 0..*d {
                l.push(None);
            }
        }
    });
    println!("{:?}", l);
    println!("{:?}", stack);


    println!("Part 1: {:?}", part1(&l, &mut stack));
    println!("Part 1 took {:?}", timer.elapsed());

    timer = Instant::now();
    println!("Part 2: {:?}", part2(&input));
    println!("Part 2 took {:?}", timer.elapsed());
}

fn part1(l: &Vec<Option<i64>>, stack:&mut LinkedList<i64>) -> i64 {
    let mut result_vec = Vec::new();
    let mut ptr = 0;
    while !stack.is_empty() {
        let curr = l.get(ptr).unwrap();
        match curr {
            Some(_) => {
                let ele = stack.pop_front().unwrap();
                result_vec.push(ele);
            }
            None => {
                let ele = stack.pop_back().unwrap();
                result_vec.push(ele)
            }
        }
        ptr += 1;
    }
    result_vec.iter().enumerate().map(|(i, num)| i as i64 * num).sum()
}

fn part2(input: &String) -> i64 {
    todo!()
}
