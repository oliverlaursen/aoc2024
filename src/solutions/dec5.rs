use std::{
    collections::{HashMap, HashSet},
    fs,
};


pub fn solve() {
    let input = fs::read_to_string("input/input5.txt").unwrap();
    let mut split = input.split("\n\n");
    let mut page_rules: HashMap<i64, Vec<i64>> = HashMap::new();
    split
        .next()
        .unwrap()
        .to_string()
        .lines()
        .map(|line| line.split("|"))
        .for_each(|mut split| {
            let x = split.next().unwrap().parse().unwrap();
            let y = split.next().unwrap().parse().unwrap();
            page_rules.entry(y).or_insert(Vec::new()).push(x)
        });
    let mut updates: Vec<Vec<i64>> = split
        .next()
        .unwrap()
        .lines()
        .map(|line| line.split(',').map(|x| x.parse().unwrap()).collect())
        .collect();
    println!("Part 1: {:?}", part1(&page_rules, &updates));
    println!("Part 2: {:?}", part2(&page_rules, &mut updates));
}

fn part1(page_rules: &HashMap<i64, Vec<i64>>, updates: &Vec<Vec<i64>>) -> i64 {
    let mut count = 0;
    for update in updates {
        if is_valid(update, page_rules){
            count += update.get(update.len()/2).unwrap();
        }
    }
    count
}

fn is_valid(update: &Vec<i64>, page_rules: &HashMap<i64, Vec<i64>>) -> bool {
    update.iter().fold(Some(HashSet::new()), |acc, &order| {
        acc.and_then(|mut seen| {
            let empty_vec = Vec::new();
            let must_see_before = page_rules.get(&order).unwrap_or(&empty_vec);
            if must_see_before.iter().any(|x| update.contains(x) && !seen.contains(x)) {
                None 
            } else {
                seen.insert(order);
                Some(seen) 
            }
        })
    }).is_some()
}


fn part2(page_rules: &HashMap<i64, Vec<i64>>, updates: &mut Vec<Vec<i64>>) -> i64 {
    let mut count = 0;
    for update in updates {
        if is_valid(update, page_rules){
            continue;
        }
        
        let mut seen = HashSet::new();
        while !is_valid(update, page_rules) {
            for i in 0..update.len() {
                let order = update[i].clone();
                let empty_vec = Vec::new();
                let must_see_before = page_rules.get(&order).unwrap_or(&empty_vec);
                let mut problem = must_see_before
                    .iter()
                    .filter(|x| update.contains(x) && !seen.contains(*x));
                if let Some(x) = problem.next(){
                    let problem_idx = update.iter().position(|y|x==y).unwrap().clone();
                    update.swap(i, problem_idx);
                    break;
                }
                seen.insert(order);
            }
        }
        count += update.get(update.len() / 2).unwrap();
    }
    count
}
