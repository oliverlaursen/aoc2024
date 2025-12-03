use std::{collections::btree_map::Range, fs, time::Instant};

pub fn solve() {
    let input = fs::read_to_string("input/2025/input2.txt").unwrap();
    let ids: Vec<usize> = input
        .split(',').into_iter()
        .flat_map(|pair| {
            let mut split = pair.split('-');
            let start:usize = split.next().unwrap().parse().unwrap();
            let end:usize = split.next().unwrap().parse().unwrap();
            start..end
            }
        )
        .collect();
    
    let mut timer = Instant::now();

    println!("Part 1: {:?}", part1(&ids));
    println!("Part 1 took {:?}", timer.elapsed());

    timer = Instant::now();
    println!("Part 2: {:?}", part2(&ids));
    println!("Part 2 took {:?}", timer.elapsed());
}

fn part1(ids: &Vec<usize>) -> usize {
    ids.iter()
        .map(|id| id.to_string())
        .map(|id| {
            let (left, right) = id.split_at(id.len() / 2);
            (left.to_string(), right.to_string())
        })
        .filter(|pair| {
            pair.0 == pair.1
        })
        .map(|pair| pair.0 + &pair.1)
        .map(|str_num| str_num.parse::<usize>().unwrap())
        .reduce(|acc, e| acc+e).unwrap()
}

fn part2(ids: &Vec<usize>) -> usize {
    ids.iter()
        .copied()
        .filter(|id| {
            let mut acc = "".to_string();
            let mut i = 0;
            let id_string = id.to_string();

            for c in id_string.chars(){
                i += 1;
                acc += &c.to_string();

                if i > id_string.len()/2 {
                    break;
                }
                let check = acc.repeat(id_string.len()/i);
                if check == id_string {
                    return true;
                }
            } 
            false
        })
        .reduce(|acc, e| acc + e).unwrap()
}