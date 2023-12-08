use core::num;
use std::collections::HashMap;

const RIGHT: usize = 0;
const LEFT: usize = 1;

fn check_if_ends_with_z(s: &Vec<&String>) -> bool {
    s.iter().all(|x| x.ends_with("Z"))
}

fn part2() {
    let input = include_str!("./input");

    let directions: Vec<_> = input
        .lines()
        .nth(0)
        .unwrap()
        .chars()
        .map(|x| match x {
            'R' => RIGHT,
            'L' => LEFT,
            _ => panic!("Invalid direction"),
        })
        .collect();

    let mut map: HashMap<String, (String, String)> = HashMap::new();

    let mut start_positions = vec![];
    let mut end_positions = vec![];
    input.lines().skip(2).for_each(|line| {
        let (key, to_parse) = line.split_once("=").unwrap();
        let key = key.trim();

        if key.ends_with("A") {
            start_positions.push(key.to_string());
        }
        if key.ends_with("Z") {
            end_positions.push(key.to_string());
        }

        let (left, right) = to_parse.split_once(",").unwrap();
        let right = right
            .chars()
            .filter(|x| x.is_alphanumeric())
            .collect::<String>();

        let left = left
            .chars()
            .filter(|x| x.is_alphanumeric())
            .collect::<String>();

        map.insert(key.to_string(), (left, right));
    });
    let mut lut: HashMap<_, _> = HashMap::new();

    start_positions.iter().for_each(|x| {
        let mut counter: u128 = 0;
        let start_position = x;
        let mut directions_iter = directions.iter().cycle();
        let mut current_position = start_position;
        let (left, right) = map.get(current_position).unwrap();
        if *directions_iter.next().unwrap() == RIGHT {
            current_position = right;
        } else {
            current_position = left;
        }
        counter += 1;

        let mut has_seen = vec![];
        while current_position != start_position {
            let (left, right) = map.get(current_position).unwrap();

            if *directions_iter.next().unwrap() == RIGHT {
                current_position = right;
            } else {
                current_position = left;
            }

            counter += 1;
            if current_position.ends_with("Z") {
                if has_seen.iter().any(|(pos, _)| *pos == current_position) {
                    println!(
                        "{} meets {} looped",
                        start_position, current_position 
                    );
                    break;
                }
                has_seen.push((current_position, counter));
            }
        }
        lut.insert(start_position, has_seen);
    });
    println!("lut: {:?}", lut);
    for (key, value) in lut.iter() {
        // put this into python
        // from math import lcm
        // lcm(11911,16897,19667,18559,13019,21883,)
        // 12833235391111
        
        println!("{}", value[0].1);
    }
    // ok this doesnt work, but made pointers work im proud
    // let mut pointer_positions = start_positions.iter().map(|x| x).collect::<Vec<_>>();
    // while !check_if_ends_with_z(&pointer_positions) {
    //     let current_direction = directions.next().unwrap();
    //     for pointer in pointer_positions.iter_mut() {
    //         let (left, right) = map.get(*pointer).unwrap();
    //         if current_direction == RIGHT {
    //             *pointer = right;
    //         } else {
    //             *pointer = left;
    //         }
    //     }
    //     counter += 1;
    //     if counter % 1_000_000 == 0 {
    //         println!("counter: {}", counter);
    //     }
    // }

    // we need to find loops, i think
}

fn part1() {
    let input = include_str!("./input");

    let directions: Vec<_> = input
        .lines()
        .nth(0)
        .unwrap()
        .chars()
        .map(|x| match x {
            'R' => RIGHT,
            'L' => LEFT,
            _ => panic!("Invalid direction"),
        })
        .collect();

    let mut map: HashMap<String, (String, String)> = HashMap::new();
    input.lines().skip(2).for_each(|line| {
        let (key, to_parse) = line.split_once("=").unwrap();
        let key = key.trim();
        let (left, right) = to_parse.split_once(",").unwrap();
        let right = right
            .chars()
            .filter(|x| x.is_alphabetic())
            .collect::<String>();
        let left = left
            .chars()
            .filter(|x| x.is_alphabetic())
            .collect::<String>();
        map.insert(key.to_string(), (left, right));
    });

    let mut counter = 0;
    let mut directions = directions.into_iter().cycle();
    let mut current_position = "AAA".to_string();

    while current_position != "ZZZ".to_string() {
        let (left, right) = map.get(&current_position).unwrap();
        if directions.next().unwrap() == RIGHT {
            current_position = right.clone();
        } else {
            current_position = left.clone();
        }
        counter += 1;
    }
    println!("counter: {}", counter);
}

fn main() {
    let start = std::time::Instant::now();
    part2();
    let end = std::time::Instant::now();
    println!("{:?}", end - start);
}
