use std::{collections::HashSet, fs, time::Instant};

struct Equation {
    expected_value: i64,
    operators: Vec<i64>,
}

impl Equation {
    pub fn adds_up(&self, part2: bool) -> bool {
        Self::calculate_combinations(&self.operators, part2).contains(&self.expected_value)
    }

    fn calculate_combinations(numbers: &[i64], part2: bool) -> HashSet<i64> {
        // Helper function to perform recursion
        fn helper(numbers: &[i64], index: usize, current: i64, results: &mut HashSet<i64>, part2: bool) {
            if index == numbers.len() {
                results.insert(current);
                return;
            }
    
            let next = numbers[index];
            // Branch: Add or Multiply
            helper(numbers, index + 1, current + next, results, part2);
            helper(numbers, index + 1, current * next, results, part2);
            if part2 {
                helper(numbers, index + 1, (current*(10_i64.pow(next.ilog10()+1))) + next, results, part2);
            }
        }
    
        let mut results = HashSet::new();
        helper(numbers, 0, 0, &mut results, part2);
        results
    }
    
}

pub fn solve() {
    let input = fs::read_to_string("input/input7.txt").unwrap();
    let equations: Vec<Equation> = input.lines().map(|line|{
        let (expected_value, operators) = line.split_once(':').unwrap();
        let expected_value = expected_value.parse().unwrap();
        let operators = operators.trim().split(' ').map(|x|x.parse().unwrap()).collect();
        Equation { expected_value, operators}
    }).collect();

    let mut timer = Instant::now();
    println!("Part 1: {:?}", part1(&equations));
    println!("Part 1 took {:?}", timer.elapsed());

    timer = Instant::now();
    println!("Part 2: {:?}", part2(&equations));
    println!("Part 2 took {:?}", timer.elapsed());
}

fn part1(equations: &Vec<Equation>) -> i64 {
    equations.iter().filter(|equation| equation.adds_up(false)).map(|x|x.expected_value).sum()
}

fn part2(equations: &Vec<Equation>) -> i64 {
    equations.iter().filter(|equation| equation.adds_up(true)).map(|x|x.expected_value).sum()

}