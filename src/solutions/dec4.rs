use std::fs;

pub fn solve() {
    let input = fs::read_to_string("input/input4.txt").unwrap();
    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
}

fn part1(input: &String) -> i64 {
    let horizontal: i64 = input.chars().collect::<Vec<char>>().windows(4).map(|word| {
        if String::from_iter(word) == "XMAS" || String::from_iter(word) == "SAMX" {
            return 1
        }
        0
    }).sum();
    let vertical_and_diagonal: i64 =    
    input
        .lines()
        .collect::<Vec<&str>>()
        .windows(4)
        .map(|block|{
            let mut score = 0;
            
            for i in 0..block[0].len()-3 {
                let first1 = block[0].chars().nth(i).unwrap();
                let second1 = block[1].chars().nth(i+1).unwrap();
                let third1 = block[2].chars().nth(i+2).unwrap();
                let fourth1 = block[3].chars().nth(i+3).unwrap();
                let diagonal_word1 = String::from_iter(vec![first1,second1,third1,fourth1]);

                let first2 = block[0].chars().nth(i+3).unwrap();
                let second2 = block[1].chars().nth(i+2).unwrap();
                let third2 = block[2].chars().nth(i+1).unwrap();
                let fourth2 = block[3].chars().nth(i).unwrap();
                let diagonal_word2 = String::from_iter(vec![first2,second2,third2,fourth2]);

                if diagonal_word1 == "XMAS" || diagonal_word1 == "SAMX" {
                    score += 1;
                }
                if diagonal_word2 == "XMAS" || diagonal_word2 == "SAMX" {
                    score += 1;
                }
            }
            for i in 0..block[0].len() {
                let first = block[0].chars().nth(i).unwrap();
                let second = block[1].chars().nth(i).unwrap();
                let third = block[2].chars().nth(i).unwrap();
                let fourth = block[3].chars().nth(i).unwrap();
                let vertical_word = String::from_iter(vec![first,second,third,fourth]);
                if vertical_word == "XMAS" || vertical_word == "SAMX" {
                    score += 1;
                }
            }
            score
        })
        .sum();
    vertical_and_diagonal + horizontal
    
}

fn part2(input: &String) -> i64 {
    input
        .lines()
        .collect::<Vec<&str>>()
        .windows(3)
        .map(|block|{
            let mut score = 0;
            
            for i in 0..block[0].len()-2 {
                let first1 = block[0].chars().nth(i).unwrap();
                let second1 = block[1].chars().nth(i+1).unwrap();
                let third1 = block[2].chars().nth(i+2).unwrap();
                let diagonal_word1 = String::from_iter(vec![first1,second1,third1]);

                let first2 = block[0].chars().nth(i+2).unwrap();
                let second2 = block[1].chars().nth(i+1).unwrap();
                let third2 = block[2].chars().nth(i).unwrap();
                let diagonal_word2 = String::from_iter(vec![first2,second2,third2]);

                if (diagonal_word1 == "MAS" || diagonal_word1 == "SAM") && (diagonal_word2 == "MAS" || diagonal_word2 == "SAM") {
                    score += 1;
                }
            }
            score
        })
        .sum()
}