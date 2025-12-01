use core::fmt;
use std::{collections::HashSet, error::Error, fmt::write, fs, time::Instant};

use grid::*;

enum MoveError {
    HitObstacleError,
    ExitBoundsError,
}

#[derive(Debug, PartialEq, Clone)]
enum Field {
    Obstacle,
    Empty,
    Guard,
    Visited,
}

#[derive(PartialEq, Hash,Eq, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down
}

struct Guard {
    pos: (usize,usize),
    direction: Direction
}

impl Guard {
    pub fn move_forward(&mut self, grid:&mut Grid<Field>) -> Result<(usize,usize), MoveError>{
        let new_pos = match self.direction {
            Direction::Left => (self.pos.0, (self.pos.1)-1),
            Direction::Right => (self.pos.0, (self.pos.1)+1),
            Direction::Up => ((self.pos.0)-1, self.pos.1),
            Direction::Down => ((self.pos.0)+1, self.pos.1),
        };
        if let Some(field) = grid.get(new_pos.0, new_pos.1) {
            if *field != Field::Obstacle {
                self.pos = new_pos;
                *grid.get_mut(new_pos.0, new_pos.1).unwrap() = Field::Visited;
                Ok(new_pos)
            }
            else {
                Err(MoveError::HitObstacleError)
            }
        } else {
            Err(MoveError::ExitBoundsError)
        }
    }

    pub fn rotate(&mut self) {
        let new_direction = match self.direction {
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
        };
        self.direction = new_direction;
    }
    
    pub fn from_grid(grid:&mut Grid<Field>) -> Option<Guard> {
        for col in 0..grid.cols(){
            for row in 0..grid.rows() {
                if *grid.get(row, col).unwrap() == Field::Guard {
                    //*grid.get_mut(row, col).unwrap() = Field::Visited;
                    return Some(Self {pos:(row, col), direction:Direction::Up})
                }
            }
        }
        None
    }
    pub fn set_pos(&mut self, pos:(usize,usize)) {
        self.pos = pos;
    }
    pub fn set_direction(&mut self, direction:Direction) {
        self.direction = direction;
    }
}

pub fn solve() {
    let input = fs::read_to_string("input/input6.txt").unwrap();    
    let rows:Vec<Field> = input.chars().filter(|c|*c != '\n').map(|c| match c {
        '.' => Field::Empty,
        '#' => Field::Obstacle,
        '^' => Field::Guard,
        _ => todo!(),
    }).collect();
    let col_size = input.lines().next().unwrap().len();
    let mut grid = Grid::from_vec(rows.clone(), col_size);

    let mut timer = Instant::now();
    println!("Part 1: {:?}", part1(&mut grid));
    println!("Part 1 took {:?}", timer.elapsed());

    timer = Instant::now();
    let mut grid = Grid::from_vec(rows, col_size);
    println!("Part 2: {:?}", part2(&mut grid));
    println!("Part 2 took {:?}", timer.elapsed());
}

fn part1(grid: &mut Grid<Field>) -> i64 {
    let mut guard = Guard::from_grid(grid).unwrap();
    let mut visited = HashSet::new();
    loop {
        let move_result = guard.move_forward(grid);
        match move_result {
            Ok(new_pos) => {
                visited.insert(new_pos);
            },
            Err(MoveError::HitObstacleError) => {
                guard.rotate();
            },
            Err(MoveError::ExitBoundsError) => break,
        }
    }
    visited.iter().count() as i64 + 1
}

fn part2(grid: &mut Grid<Field>) -> i64 {
    let mut positions_to_try = Vec::new();
    let mut stuck_count = 0;
    let mut guard = Guard::from_grid(grid).unwrap();
    let initial_pos = guard.pos;
    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            let field = grid.get(row, col).unwrap();
            if *field != Field::Obstacle && *field != Field::Guard {
                positions_to_try.push((row,col));
            }
        }
    }
    for pos in positions_to_try {
        guard.set_pos(initial_pos);
        guard.set_direction(Direction::Up);
        *grid.get_mut(pos.0, pos.1).unwrap() = Field::Obstacle;

        let mut positions: HashSet<((usize,usize), Direction)> = HashSet::new();
        positions.insert((initial_pos,Direction::Up));
        loop {
            let move_result = guard.move_forward(grid);
            match move_result {
                Ok(new_pos) => {
                    if positions.contains(&(new_pos, guard.direction.clone())) {
                        stuck_count += 1;
                        break;
                    }
                    positions.insert((new_pos,guard.direction.clone()));
                },
                Err(MoveError::HitObstacleError) => {
                    guard.rotate();
                },
                Err(MoveError::ExitBoundsError) => break,
            }
        }
        *grid.get_mut(pos.0, pos.1).unwrap() = Field::Empty;
    }
    stuck_count
}