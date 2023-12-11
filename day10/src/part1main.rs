use lazy_static::lazy_static;
use std::borrow::BorrowMut;
use std::cell::RefCell;
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
    neighbors: Vec<Rc<RefCell<Node>>>,
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
    fn add_neighbor(&mut self, neighbor: Rc<RefCell<Node>>) {
        self.neighbors.push(neighbor);
    }
}

fn bfs<'a>(start: Rc<RefCell<Node>>, width: usize, height: usize) -> usize {
    let mut queue = Vec::new();
    let mut visited: Vec<Rc<RefCell<Node>>> = Vec::new();
    let mut distance_map = vec![vec![0; width]; height];
    queue.push(start);

    while !queue.is_empty() {
        let node_rc = queue.remove(0);
        let node = node_rc.borrow();

        for neighbor in &node.neighbors {
            let neighbor_rc = neighbor.clone();
            let neighbor_borrowed = neighbor_rc.borrow();

            if !visited.contains(&neighbor) {
                distance_map[neighbor_borrowed.positon.1 as usize]
                    [neighbor_borrowed.positon.0 as usize] =
                    distance_map[node.positon.1 as usize][node.positon.0 as usize] + 1;
                
                queue.push(neighbor.clone());
            }
        }
        visited.push(node_rc.clone());
    }
    // for i in distance_map.iter() {
    //     println!("{:?}", i);
    // }
    return distance_map.iter().flatten().max().unwrap().clone() as usize;
}

fn part1() {
    let nodes: Vec<Vec<Rc<RefCell<Node>>>> = MAP
        .iter()
        .enumerate()
        .map(|y| {
            y.1.iter()
                .enumerate()
                .map(|x| Node::new((x.0, y.0)))
                .collect()
        })
        .collect();

    MAP.iter().enumerate().for_each(|(y, chars)| {
        chars.iter().enumerate().for_each(|(x, c)| {
            let neighbours = process_char(*c, x as i32, y as i32);
            let neigbors: Vec<_> = neighbours
                .iter()
                .map(|coord| nodes[coord.1][coord.0].clone())
                .collect();
            (*nodes[y][x]).borrow_mut().neighbors = neigbors;
        })
    });

    let mut start_pos = (0, 0);
    'outer: for (y_idx, x) in MAP.iter().enumerate() {
        for (x_idx, y) in x.iter().enumerate() {
            if y == &'S' {
                start_pos = (x_idx, y_idx);
                break 'outer;
            }
        }
    }

    let start_node = nodes
        .iter()
        .flatten()
        .find(|x| x.borrow().positon == start_pos)
        .unwrap();

    let res = bfs(start_node.clone(), MAP[0].len(), MAP.len());
    println!("res: {}", res);
}

fn check_neighbors<'a>(x: i32, y: i32, directions: [Direction; 2]) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    // println!("x: {}, y: {}", x, y);
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
        
        if MAP[y][x] != '.' {
            let opposite = direction.get_opposite_direction();
            let neighbor_directions = process_char_dir(MAP[y][x]);
            // println!("\nneighbor_directions: {:?}", neighbor_directions);
            // println!("opposite: {:?}", opposite);
            if neighbor_directions.contains(&opposite) {
                // println!("added\n");
                neighbors.push((x, y));
            }
        }
    }
    return neighbors;
}

// | is a vertical pipe connecting north and south.
// - is a horizontal pipe connecting east and west.
// L is a 90-degree bend connecting north and east.
// J is a 90-degree bend connecting north and west.
// 7 is a 90-degree bend connecting south and west.
// F is a 90-degree bend connecting south and east.
// . is ground; there is no pipe in this tile.
// S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
const START_CHAR: char = '-';

fn process_char(c: char, x: i32, y: i32) -> Vec<(usize, usize)> {
    // we can cheat with S because we know it's the only one, and we know the input hehe
    match c {
        '|' => return check_neighbors(x, y, [Direction::North, Direction::South]),
        '-' => return check_neighbors(x, y, [Direction::East, Direction::West]),
        'L' => return check_neighbors(x, y, [Direction::North, Direction::East]),
        'J' => return check_neighbors(x, y, [Direction::North, Direction::West]),
        '7' => return check_neighbors(x, y, [Direction::South, Direction::West]),
        'F' => return check_neighbors(x, y, [Direction::South, Direction::East]),
        '.' => return vec![],
        'S' => process_char(START_CHAR, x, y), // test is F
        _ => panic!("Unknown char"),
    }
}
fn process_char_dir(c: char) -> [Direction; 2] {
    // we can cheat with S because we know it's the only one, and we know the input hehe
    match c {
        '|' => return [Direction::North, Direction::South],
        '-' => return [Direction::East, Direction::West],
        'L' => return [Direction::North, Direction::East],
        'J' => return [Direction::North, Direction::West],
        '7' => return [Direction::South, Direction::West],
        'F' => return [Direction::South, Direction::East],
        '.' => return [Direction::Invalid, Direction::Invalid],
        'S' => process_char_dir(START_CHAR), // test is F
        _ => panic!("Unknown char"),
    }
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
