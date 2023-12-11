use lazy_static::lazy_static;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::{Rc, Weak};

#[derive(PartialEq, Eq, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
    Invalid,
}

impl Direction {
    fn get_map_corresponding_to_direction(&self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
            Direction::Invalid => (0, 0),
        }
    }

    fn get_opposite_direction(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
            Direction::Invalid => Direction::Invalid,
        }
    }
}

#[derive(Clone, Debug)]
struct Node {
    neighbors: Vec<Weak<RefCell<Node>>>,
    positon: (usize, usize),
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.positon == other.positon
    }
}

impl Node {
    fn new(positon: (usize, usize)) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            neighbors: vec![],
            positon,
        }))
    }
    fn add_neighbor(&mut self, neighbor: Weak<RefCell<Node>>) {
        self.neighbors.push(neighbor);
    }
}

fn part1() {
    let input = include_str!("./input");

    let map: Vec<_> = input
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();

    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    for (y, row) in map.iter().enumerate() {
        println!("y: {}", y);
        let char = row[0];
        println!("char: {}", char);
        if char != '.' {
            continue;
        }

        let mut neighbors = vec![(0, y)];
        while neighbors.len() > 0 {
            let (x, y) = neighbors.pop().unwrap();
            visited.insert((x, y));
            let current_neighbours = check_neighbors(
                x as i32,
                y as i32,
                [
                    Direction::South,
                    Direction::East,
                    Direction::West,
                    Direction::North,
                ],
            );
            for neighbor in current_neighbours {
                visited.insert(neighbor);
            }
        }
    }

    println!("visited: {}", visited.len());
    let total = map.iter().flatten().filter(|x| **x == '.').count();

    println!("total: {}", total);
    println!("res: {}", total - visited.len());
}

fn check_neighbors<'a>(x: i32, y: i32, directions: [Direction; 4]) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    for direction in directions {
        let (x_offset, y_offset) = direction.get_map_corresponding_to_direction();
        let x = (x + x_offset) as usize;
        let y = (y + y_offset) as usize;
        if MAP.get(y).is_none() {
            continue;
        }

        if MAP[y].get(x).is_none() {
            continue;
        }

        if MAP[y][x] == '.' {
            neighbors.push((x, y));
        }
    }
    return neighbors;
}

lazy_static! {
    static ref MAP: Vec<Vec<char>> = {
        let input = include_str!("./input");
        input
            .lines()
            .map(|x| x.chars().collect::<Vec<char>>())
            .collect()
    };
}

fn main() {
    let start = std::time::Instant::now();
    part1();
    let end = std::time::Instant::now();
    println!("{:?}", (end - start));
}
