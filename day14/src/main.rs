use std::{
    collections::HashMap,
    ops::{Add, Deref},
};

static mut MAP: Vec<Vec<char>> = vec![];
static mut WIDTH: usize = 0;
static mut HEIGHT: usize = 0;
fn init() {
    let input = include_str!("./input");
    unsafe {
        MAP = input
            .lines()
            .map(|x| x.chars().collect::<Vec<char>>())
            .collect();
        WIDTH = MAP[0].len();
        HEIGHT = MAP.len();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

#[inline(always)]
fn can_move(to_point: &(i64, i64)) -> bool {
    if to_point.0 < 0
        || to_point.1 < 0
        || to_point.0 >= unsafe { WIDTH } as i64
        || to_point.1 >= unsafe { HEIGHT } as i64
    {
        return false;
    }

    let x = to_point.0 as usize;
    let y = to_point.1 as usize;

    return *unsafe { MAP.get_unchecked(y).get_unchecked(x) } == '.';
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Point {
    #[inline(always)]
    fn move_to(&mut self, dir: &(i64, i64)) {
        let mut point = Point {
            x: self.x + dir.0,
            y: self.y + dir.1,
        };
        let mut moved = false;
        loop {
            if can_move(&(point.x, point.y)) {
                point.x += dir.0;
                point.y += dir.1;
                moved = true;
            } else {
                point.x -= dir.0;
                point.y -= dir.1;
                break;
            }
        }
        // println!("{:?}", moved);
        if moved {
            unsafe {
                MAP[self.y as usize][self.x as usize] = '.';
            }

            self.x = point.x;
            self.y = point.y;
            unsafe {
                MAP[self.y as usize][self.x as usize] = 'O';
            }
        }
    }

    fn score(&self, map: &Vec<Vec<char>>) -> i64 {
        let score = map.len() as i64 - self.y;
        return score;
    }
}

fn part1() {
    init();
    let mut vec = vec![];

    unsafe {
        for i in 0..MAP.len() {
            for j in 0..MAP[i].len() {
                if MAP[i][j] == 'O' {
                    vec.push(Point {
                        x: j as i64,
                        y: i as i64,
                    });
                }
            }
        }
    }

    let directions = vec![(0i64, -1), (-1, 0), (0, 1), (1, 0)];
    // let directions = vec![(0,-1), (0,1)];
    let mut cache: HashMap<Vec<Vec<char>>, usize> = HashMap::new();
    let total = 1_000_000_000;
    let mut winner = vec![];
    for i in 0..4 {
        println!("i {}", i);
        for direction in &directions {
            if direction.0 == 0 {
                vec.sort_by(|a, b| a.x.cmp(&b.x));
                vec.iter_mut().for_each(|x| x.move_to(&direction));
            }
            else {
                vec.sort_by(|a, b| a.y.cmp(&b.y));
                vec.iter_mut().for_each(|x| x.move_to(&direction));
            }
        }
        let cloned = unsafe { MAP.clone() };
        for i in &cloned {
            for j in i {
                print!("{}", j);
            }
            println!();
        }
        println!();
        let cloned = unsafe { MAP.clone() };
        match cache.insert(cloned, i) {
            Some(x) => {
                println!("x {x} i {i}");
                let cycle_len = i - x;
                println!("cycle_len {}", cycle_len);
                let position = i - (total) % cycle_len;
                println!("position {}", position);
                match cache.iter().find(|x| *x.1 == position) {
                    Some(x) => {
                        winner = x.0.clone();
                        break;
                    }
                    None => {
                        continue;
                        // break;
                    }
                };
            }
            None => {
                continue;
            }
        }
    }
    // let mut debug_map = map.clone();
    // debug_map.iter_mut().flatten().for_each(|x| {
    //     if *x == 'O' {
    //         *x = '.'
    //     }
    // });
    // for i in vec.iter() {
    //     debug_map[i.y as usize][i.x as usize] = 'O';
    // }
    // for i in debug_map.iter() {
    //     for j in i.iter() {
    //         print!("{}", j);
    //     }
    //     println!();
    // }
    // println!();
    let mut winners = vec![];
    for i in 0..winner.len() {
        for j in 0..winner[i].len() {
            if winner[i][j] == 'O' {
                winners.push(Point {
                    x: j as i64,
                    y: i as i64,
                });
            }
        }
    }
    let sum: i64 = winners.iter().map(|x| x.score(&winner)).sum();
    println!("{:?}", sum);
}

fn main() {
    let start = std::time::Instant::now();
    part1();
    let end = std::time::Instant::now();
    println!("{:?}", (end - start));
}
