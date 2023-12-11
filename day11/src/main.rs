
fn rotate(input: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut output = vec![vec![];input[0].len()];

    for y in 0..input.len() {
        for x in 0..input[0].len() {
            output[x].push(input[y][x]);
        }
    }
    return output;
}

fn expand(input: Vec<Vec<char>>) -> Vec<Vec<char>>  {
    let mut output = vec![];
    for i in input {
        let should_expand = i.iter().all(|x| *x == '.');
        if should_expand {
            output.push(i.clone());
            output.push(i.clone());
        } else {
            output.push(i.clone());
        }
    }
    let rotated = rotate(&output); 
    let mut output = vec![];
    for i in rotated {
        let should_expand = i.iter().all(|x| *x == '.');
        if should_expand {
                output.push(i.clone());
                output.push(i.clone());
        } else {
            output.push(i.clone());
        }
    }
    return output;
}
#[derive(Debug,Clone,Copy,PartialEq, Eq)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn distance(&self, other: &Position) -> i64 {
        return (self.x - other.x).abs() + (self.y - other.y).abs();
    }
}

const CONSTANT:i64 = 1_000_000;

fn expand_part2(input: Vec<Vec<char>>)-> Vec<Position> {
    let mut y_coords = vec![];
    let mut actual_y = 0;
    for i in &input {
        let should_expand = i.iter().all(|x| *x == '.');
        y_coords.push(actual_y);
        if should_expand {
            actual_y += CONSTANT;
        } else {
            actual_y += 1;
        }
    }
    let mut x_coords = vec![];
    let mut actual_x = 0;
    let rotated = rotate(&&input); 

    for i in rotated {
        let should_expand = i.iter().all(|x| *x == '.');
        if should_expand {
            x_coords.push(actual_x);
            actual_x += CONSTANT;
        } else {
            x_coords.push(actual_x);
            actual_x += 1;
        }
    }

    let mut positions = vec![];
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if input[y][x] == '#' {
                positions.push(Position{x: x_coords[x] as i64, y: y_coords[y] as i64});
            }
        }
    }
    return positions;
}

fn part2() {
    let input = include_str!("./input");
    let grid = input.lines().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>(); 
    let positions = expand_part2(grid);

    let mut sum = 0;
    for i in &positions {
        let min_distance: i64 = positions.iter().map(|x| i.distance(x)).sum();
        sum += min_distance;
    } 
    println!("{:?}", sum/2);
}

fn part1() {
    let input = include_str!("./input");
    let grid = input.lines().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>(); 
    let expanded = expand(grid);
    let mut positions = vec![];
    for y in 0..expanded.len() {
        for x in 0..expanded[0].len() {
            if expanded[y][x] == '#' {
                positions.push(Position{x: x as i64, y: y as i64});
            }
        }
    }

    let mut sum = 0;
    for i in &positions {
        let min_distance: i64 = positions.iter().map(|x| i.distance(x)).sum();
        sum += min_distance;
    } 
    println!("{:?}", sum/2);
}

fn main() {
    let start = std::time::Instant::now();
    part2();
    let end = std::time::Instant::now();
    println!("{:?}", (end - start));
}
