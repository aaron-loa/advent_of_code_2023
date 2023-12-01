use std::cmp::min;

fn part_one() {
    let input = include_str!("./input_a");
    let lines = input.lines();
    let sum: u32 = lines
        .map(|line| {
            let first = line.chars().find_map(|x| x.to_digit(10)).unwrap();
            let last = line.chars().rev().find_map(|x| x.to_digit(10)).unwrap();
            return first * 10 + last;
        })
        .sum();

    println!("{sum}");
}

fn part_two() {
    let input = include_str!("./input_a");
    let lines = input.lines();
    let map = [
        ("one".to_string(), 1),
        ("two".to_string(), 2),
        ("three".to_string(), 3),
        ("four".to_string(), 4),
        ("five".to_string(), 5),
        ("six".to_string(), 6),
        ("seven".to_string(), 7),
        ("eight".to_string(), 8),
        ("nine".to_string(), 9),
    ];

    let sum: u32 = lines
        .map(|line| {
            let mut first = 0;
            let mut last = 0;
            for (idx, char) in line.chars().enumerate() {
                first = char.to_digit(10).unwrap_or(0);
                if first != 0 {
                    break;
                }
                first += map
                    .iter()
                    .find_map(|x| {
                        let top_bound = min(idx + x.0.len(), line.len() - 1);
                        if line[idx..top_bound].to_string() == *x.0 {
                            return Some(x.1);
                        } else {
                            None
                        }
                    })
                    .unwrap_or(0);

                if first != 0 {
                    break;
                }
            }

            for (idx, char) in line.chars().rev().enumerate() {
                last = char.to_digit(10).unwrap_or(0);
                if first != 0 {
                    break;
                }
                last = map
                    .iter()
                    .rev()
                    .find_map(|x| {
                        let real_index = line.len() - 1 - idx;
                        let top_bound = min(real_index + x.0.len(), line.len());
                        if line[real_index..top_bound].to_string() == *x.0 {
                            return Some(x.1);
                        } else {
                            return None;
                        }
                    })
                    .unwrap_or(0);
                if last != 0 {
                    break;
                }
            }
            return first * 10 + last;
        })
        .sum();
    println!("{sum}");
}

fn part_two_easy() {}
fn main() {
    part_two();
}
