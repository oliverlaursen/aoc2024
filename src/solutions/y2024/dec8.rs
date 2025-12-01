use core::fmt;
use std::{
    collections::{HashMap, HashSet},
    fs,
    iter::Map,
    path::Display,
    time::Instant,
};

use grid::{grid, Grid};

type Pos = (usize, usize);

#[derive(Clone, PartialEq)]
enum Field {
    Empty,
    Antinode,
    Antenna(char),
}

struct AntennaCollection {
    positions: Vec<Pos>,
}

impl AntennaCollection {
    pub fn get_antinodes(&self, rows: usize, cols: usize) -> HashSet<Pos> {
        let mut antinodes = HashSet::new();
        for pos1 in &self.positions {
            for pos2 in &self.positions {
                if pos1 == pos2 {
                    continue;
                }
                let antinode_x = pos1.0 - (pos2.0 - pos1.0);
                let antinode_y = pos1.1 - (pos2.1 - pos1.1);

                if antinode_x < rows && antinode_y < cols {
                    antinodes.insert((antinode_x, antinode_y));
                }
            }
        }
        antinodes
    }

    pub fn get_antinodes_part2(&self, rows: usize, cols: usize) -> HashSet<Pos> {
        let mut antinodes = HashSet::new();
        for pos1 in &self.positions {
            for pos2 in &self.positions {
                if pos1 == pos2 {
                    continue;
                }
                let mut antinode_x = pos1.0 - (pos2.0 - pos1.0);
                let mut antinode_y = pos1.1 - (pos2.1 - pos1.1);

                while antinode_x < rows && antinode_y < cols {
                    antinodes.insert((antinode_x, antinode_y));
                    antinode_x -= pos2.0 - pos1.0;
                    antinode_y -= pos2.1 - pos1.1;
                }
            }
        }
        antinodes
    }
}

pub fn solve() {
    let input = fs::read_to_string("input/input8.txt").unwrap();
    let mut timer = Instant::now();
    let col_size = input.lines().next().unwrap().len();
    let rows: Vec<Field> = input
        .chars()
        .filter(|c| *c != '\n')
        .map(|c| match c {
            '.' => Field::Empty,
            '#' => Field::Antinode,
            x => Field::Antenna(x),
        })
        .collect();
    let mut grid = Grid::from_vec(rows, col_size);

    let mut antennas: HashMap<char, AntennaCollection> = HashMap::new();
    grid.indexed_iter()
        .for_each(|((row, col), field)| match field {
            Field::Antenna(x) => antennas
                .entry(*x)
                .or_insert(AntennaCollection {
                    positions: Vec::new(),
                })
                .positions
                .push((row, col)),
            _ => (),
        });
    let antennas: Vec<AntennaCollection> = antennas.into_values().collect();

    println!("Parseing took {:?}", timer.elapsed());
    timer = Instant::now();

    println!("Part 1: {:?}", part1(&mut grid, &antennas));
    println!("Part 1 took {:?}", timer.elapsed());

    timer = Instant::now();
    println!("Part 2: {:?}", part2(&mut grid, &antennas));
    println!("Part 2 took {:?}", timer.elapsed());
}

fn part1(grid: &mut Grid<Field>, antennas: &Vec<AntennaCollection>) -> usize {
    let antinode_positions: HashSet<Pos> = antennas
        .iter()
        .flat_map(|ant| ant.get_antinodes(grid.rows(), grid.cols()))
        .collect();
    antinode_positions.len()
}

fn part2(grid: &mut Grid<Field>, antennas: &Vec<AntennaCollection>) -> usize {
    let mut antinode_positions: HashSet<Pos> = antennas
        .iter()
        .flat_map(|ant| ant.get_antinodes_part2(grid.rows(), grid.cols()))
        .collect();
    let antenna_antinodes: Vec<Pos> = antennas
        .iter()
        .filter(|x| x.positions.len() > 1)
        .flat_map(|x| x.positions.clone())
        .collect();
    antinode_positions.extend(antenna_antinodes);
    antinode_positions.len()
}
