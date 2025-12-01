use std::{fs, time::Instant};

type Position = (usize,usize);
type Velocity = (usize,usize);

struct Robot {
    pub pos:Position,
    pub velocity: Velocity,
}

impl Robot {
    pub fn get_end_position(&self, width: usize, height: usize) -> Position {
        let mut current_pos = self.pos;
        for i in 0..100 {
            let new_x = current_pos.0 + self.velocity.0;
            let new_y = current_pos.1 + self.velocity.1;
            let new_pos = Self::handle_teleport((new_x, new_y), width, height);
            current_pos = new_pos
        }
        current_pos
    }

    fn handle_teleport(new_pos: Position, width: usize, height: usize) -> Position{
        new_pos
    }
}

pub fn solve() {
    let input = fs::read_to_string("input/input14.txt").unwrap();
    let mut timer = Instant::now();

    println!("Part 1: {:?}", part1(&input));
    println!("Part 1 took {:?}", timer.elapsed());

    timer = Instant::now();
    println!("Part 2: {:?}", part2(&input));
    println!("Part 2 took {:?}", timer.elapsed());
}

fn part1(input: &String) -> i64 {
    let robots: Vec<Robot> = input.lines().map(|line| {
        let split = line.split('v');
        let pos = &split.next().unwrap()[2..].trim();
        let pos_split = pos.split(',');
        let pos = (pos_split.next().unwrap().parse().unwrap(), pos_split.next().unwrap().parse().unwrap());

        let pos = &split.next().unwrap()[2..].trim();
        let pos_split = pos.split(',');
        let pos = (pos_split.next().unwrap().parse().unwrap(), pos_split.next().unwrap().parse().unwrap());
        Robot {pos,velocity: pos}
    }).collect();
}

fn part2(input: &String) -> i64 {
    todo!()
}