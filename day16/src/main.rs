use std::{collections::HashSet, io};

use lazy_static::lazy_static;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

#[inline(always)]
fn can_move(to_point: &(i64, i64)) -> bool {
    return !(to_point.0 < 0
        || to_point.1 < 0
        || to_point.0 >= MAP.len() as i64
        || to_point.1 >= MAP[0].len() as i64);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn to_char(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }
}

#[derive(Debug, Clone)]
struct Beam {
    direction: Direction,
    position: (i64, i64),
    can_move: bool,
}

impl Beam {
    fn new(direction: Direction, position: (i64, i64)) -> Self {
        Self {
            direction,
            position,
            can_move: true,
        }
    }

    fn move_beam(&mut self, moves: &HashSet<(i64, i64, Direction)>) -> CanMoveReturn {
        let (x, y) = self.position;
        let (x, y) = match self.direction {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        };

        self.can_move = can_move(&(x, y));
        let mut split = None;

        if self.can_move {
            self.position = (x, y);
            let current_char = MAP[y as usize][x as usize];
            match current_char {
                '\\' => {
                    self.direction = match self.direction {
                        Direction::Up => Direction::Left,
                        Direction::Down => Direction::Right,
                        Direction::Left => Direction::Up,
                        Direction::Right => Direction::Down,
                    };
                }
                '/' => {
                    self.direction = match self.direction {
                        Direction::Up => Direction::Right,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Down,
                        Direction::Right => Direction::Up,
                    };
                }
                '|' => {
                    match self.direction {
                        Direction::Left | Direction::Right => {
                            self.direction = Direction::Up;
                            split = Some(Direction::Down);
                        }
                        _ => (),
                    };
                }
                '-' => {
                    match self.direction {
                        Direction::Up | Direction::Down => {
                            self.direction = Direction::Left;
                            split = Some(Direction::Right);
                        }
                        _ => (),
                    };
                }
                _ => {}
            }
        }

        match moves.get(&(self.position.0, self.position.1, self.direction.clone())) {
            Some(_) => self.can_move = false,
            _ => (),
        }

        return CanMoveReturn {
            split,
            position: self.position,
        };
    }
}
#[derive(Debug)]
struct CanMoveReturn {
    split: Option<Direction>,
    position: (i64, i64),
}
fn part2() {
    let mut starting_beams = vec![];
    for i in 0..MAP.len() {
        // left column direction is TO RIGHT
        let current_char = MAP[i][0];
        let (x, y) = (0, i as i64);
        match current_char {
            '\\' => {
                starting_beams.push(Beam::new(Direction::Down, (x, y)));
            }
            '/' => {
                starting_beams.push(Beam::new(Direction::Up, (x, y)));
            }
            '-' => {
                starting_beams.push(Beam::new(Direction::Right, (x, y)));
            }
            '|' => {
                starting_beams.push(Beam::new(Direction::Up, (x, y)));
                starting_beams.push(Beam::new(Direction::Down, (x, y)));
            }
            '.' => {
                starting_beams.push(Beam::new(Direction::Right, (x, y)));
            }
            _ => continue,
        }
    }

    for i in 0..MAP.len() {
        // right column direction is TO LEFT
        let current_char = MAP[i][MAP[0].len() - 1];
        let (x, y) = ((MAP[0].len() - 1) as i64, i as i64);
        match current_char {
            '\\' => {
                starting_beams.push(Beam::new(Direction::Up, (x, y)));
            }
            '/' => {
                starting_beams.push(Beam::new(Direction::Down, (x, y)));
            }
            '|' => {
                starting_beams.push(Beam::new(Direction::Up, (x, y)));
                starting_beams.push(Beam::new(Direction::Down, (x, y)));
            }
            '-' => {
                starting_beams.push(Beam::new(Direction::Left, (x, y)));
            }
            '.' => {
                starting_beams.push(Beam::new(Direction::Left, (x, y)));
            }
            _ => {}
        }
    }

    for i in 0..MAP[0].len() {
        // top row direction is TO DOWN
        let current_char = MAP[0][i];
        let (x, y) = (i as i64, 0);
        match current_char {
            '\\' => {
                starting_beams.push(Beam::new(Direction::Right, (x, y)));
            }
            '/' => {
                starting_beams.push(Beam::new(Direction::Left, (x, y)));
            }
            '|' => {
                starting_beams.push(Beam::new(Direction::Down, (x, y)));
            }
            '-' => {
                starting_beams.push(Beam::new(Direction::Left, (x, y)));
                starting_beams.push(Beam::new(Direction::Right, (x, y)));
            }
            '.' => {
                starting_beams.push(Beam::new(Direction::Down, (x, y)));
            }
            _ => {}
        }
    }
    for i in 0..MAP[0].len() {
        // bottom row direction is TO UP
        let current_char = MAP[MAP.len() - 1][i];
        let (x, y) = (i as i64, (MAP.len() - 1) as i64);
        match current_char {
            '\\' => {
                starting_beams.push(Beam::new(Direction::Left, (x, y)));
            }
            '/' => {
                starting_beams.push(Beam::new(Direction::Right, (x, y)));
            }
            '|' => {
                starting_beams.push(Beam::new(Direction::Up, (i as i64, (MAP.len() - 1) as i64)));
            }
            '-' => {
                starting_beams.push(Beam::new(Direction::Left, (x, y)));
                starting_beams.push(Beam::new(Direction::Right, (x, y)));
            }
            '.' => {
                starting_beams.push(Beam::new(Direction::Up, (x, y)));
            }
            _ => {}
        }
    }
    for i in 0..MAP[0].len() {
        // bottom row direction is TO UP
        let current_char = MAP[MAP.len() - 1][i];
        let (x, y) = (i as i64, (MAP.len() - 1) as i64);
        match current_char {
            '\\' => {
                starting_beams.push(Beam::new(Direction::Left, (x, y)));
            }
            '/' => {
                starting_beams.push(Beam::new(Direction::Right, (x, y)));
            }
            '|' => {
                starting_beams.push(Beam::new(Direction::Up, (i as i64, (MAP.len() - 1) as i64)));
            }
            '-' => {
                starting_beams.push(Beam::new(Direction::Left, (x, y)));
                starting_beams.push(Beam::new(Direction::Right, (x, y)));
            }
            _ => {}
        }
    }

    let max = starting_beams
        .par_iter()
        .map(|starting_beam| {
            let mut beams = vec![];
            let mut positions = HashSet::new();
            beams.push(starting_beam.clone());
            positions.insert((
                starting_beam.position.0,
                starting_beam.position.1,
                starting_beam.direction.clone(),
            ));
            while beams.iter().any(|x| x.can_move) {
                let mut new_beams = vec![];
                for beam in beams.iter_mut() {
                    let can_move_return = beam.move_beam(&positions);

                    if let Some(split_direction) = can_move_return.split {
                        new_beams.push(Beam::new(split_direction, can_move_return.position));
                    }

                    if beam.can_move {
                        positions.insert((
                            beam.position.0,
                            beam.position.1,
                            beam.direction.clone(),
                        ));
                    }
                }
                beams.extend(new_beams);
                beams = beams
                    .into_iter()
                    .filter(|x| x.can_move)
                    .collect::<Vec<Beam>>();
            }
            let set: HashSet<(i64, i64)> = positions.iter().map(|x| (x.0, x.1)).collect();
            set.len()
        })
        .max()
        .unwrap();

    println!("Part 2: {}", max);
}

fn part1() {
    let input = include_str!("input");
    let mut beams = vec![];
    let mut positions = HashSet::new();

    beams.push(Beam {
        direction: Direction::Down,
        position: (0, 0),
        can_move: true,
    });
    positions.insert((0, 0, Direction::Down));
    // let mut debug_map = MAP.clone();
    while beams.iter().any(|x| x.can_move) {
        let mut new_beams = vec![];
        for beam in beams.iter_mut() {
            let can_move_return = beam.move_beam(&positions);

            if let Some(split_direction) = can_move_return.split {
                new_beams.push(Beam::new(split_direction, can_move_return.position));
            }

            if beam.can_move {
                positions.insert((beam.position.0, beam.position.1, beam.direction.clone()));
            }

            // debug_map[beam.position.1 as usize][beam.position.0 as usize] =
            //     beam.direction.to_char();
        }
        beams.extend(new_beams);
        beams = beams
            .into_iter()
            .filter(|x| x.can_move)
            .collect::<Vec<Beam>>();
        // for i in debug_map.iter() {
        //     for j in i.iter() {
        //         print!("{}", j);
        //     }
        //     println!();
        // }
        // let mut input = String::new();
        // io::stdin().read_line(&mut input).unwrap();
    }

    // println!("MAP");
    // for i in MAP.iter() {
    //     for j in i.iter() {
    //         print!("{}", j);
    //     }
    //     println!();
    // }

    // for i in positions.iter() {
    //     debug_map[i.1 as usize][i.0 as usize] = 'O';
    // }
    // println!("debug_MAP");
    // for i in debug_map.iter() {
    //     for j in i.iter() {
    //         print!("{}", j);
    //     }
    //     println!();
    // }
    let set: HashSet<(i64, i64)> = positions.iter().map(|x| (x.0, x.1)).collect();
    println!("Part 1: {}", set.len());
}

fn main() {
    let start = std::time::Instant::now();
    part2();
    let end = std::time::Instant::now();
    println!("{:?}", end - start);
}
