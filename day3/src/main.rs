use std::collections::{HashMap, HashSet};

use lazy_static::lazy_static;

lazy_static! {
    static ref MAP: Vec<Vec<char>> = {
        let input = include_str!("./input");
        input
            .lines()
            .map(|x| x.chars().collect::<Vec<char>>())
            .collect()
    };
}

fn check_rectangle_part_1(from_x: i32, from_y: i32, to_x: i32, to_y: i32) -> bool {
    for y in from_y..=to_y {
        for x in from_x..to_x {
            let y = y as usize;
            let x = x as usize;
            if MAP.get(y).is_none() {
                continue;
            }
            if MAP[y].get(x).is_none() {
                continue;
            }

            if !MAP[y][x].is_digit(10) && MAP[y][x] != '.' {
                return true;
            }
        }
    }
    return false;
}

fn find_number(y: usize, x: usize) -> Option<usize> {
    if !MAP[y][x].is_digit(10) {
        return None;
    }

    let left = x - MAP[y][..x]
        .iter()
        .rev()
        .take_while(|x| x.is_digit(10))
        .count();

    let right = x + MAP[y][x + 1..]
        .iter()
        .take_while(|x| x.is_digit(10))
        .count();

    let string = MAP[y][left..=right]
        .iter()
        .fold("".to_string(), |mut string, char| {
            string.push(*char);
            return string;
        });

    let res = string.parse::<usize>().unwrap();
    println!("{res}");
    return Some(res);
}

fn check_rectangle_part_2(from_x: i32, from_y: i32, to_x: i32, to_y: i32) -> Option<usize> {
    let mut set = HashSet::new();
    for y in from_y..=to_y {
        for x in from_x..=to_x {
            let y = y as usize;
            let x = x as usize;
            if MAP.get(y).is_none() {
                continue;
            }
            if MAP[y].get(x).is_none() {
                continue;
            }
            if let Some(x) = find_number(y, x) {
                set.insert(x);
            }
        }
    }

    println!("len of set: {}", set.len());

    if set.len() == 2 {
        return Some(set.iter().product());
    }
    return None;
}

fn part2() {
    let res: usize = MAP
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(x, char)| {
                    if *char == '*' {
                        return check_rectangle_part_2(
                            x as i32 - 1,
                            y as i32 - 1,
                            x as i32 + 1,
                            y as i32 + 1,
                        );
                    }
                    None
                })
                .sum::<usize>()
        })
        .sum();
    println!("{}", res);
}

fn part1() {
    let char = vec!['.'];
    let expand = char.iter();
    let res: usize = MAP
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .chain(expand.clone())
                .enumerate()
                .fold(("".to_string(), 0), |(mut state, mut sum), (x, char)| {
                    if char.is_digit(10) {
                        state.push(*char);
                    } else if state.len() != 0 {
                        let value = match state.parse::<usize>() {
                            Ok(x) => x,
                            _ => return (state, sum),
                        };
                        let from_x = x as i32 - state.len() as i32 - 1;
                        let from_y = y as i32 - 1;
                        println!("{state} {value}, len {}, x {x}, y {y}", state.len());
                        if check_rectangle_part_1(from_x, from_y, (x + 1) as i32, (y + 1) as i32) {
                            sum += value;
                        }
                        state = "".to_string();
                    } else {
                        state = "".to_string();
                    }
                    return (state, sum);
                })
                .1
        })
        .sum();
    println!("{}", res);
}

fn main() {
    part2();
}
