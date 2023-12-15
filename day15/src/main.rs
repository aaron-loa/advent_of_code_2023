use std::{array, collections::LinkedList};

fn parse(input: &str) -> usize {
    let start = 0;
    let res = input.chars().fold(start, |mut acc, x| {
        let ascii_value = x as usize;
        acc = acc + ascii_value;
        acc = acc * 17;
        acc = acc % 256;
        return acc;
    });
    return res;
}
fn part1() {
    let input = include_str!("./input");
    let input = input.replace("\n", "");
    let sum: usize = input.split(",").into_iter().map(|x| parse(x)).sum();
    println!("sum: {}", sum);
}

#[derive(Debug)]
struct Entry {
    label: String,
    value: usize,
}

fn part2() {
    let input = include_str!("./input");
    let input = input.replace("\n", "");
    let mut map: [Vec<Entry>; 256] = array::from_fn(|_| Vec::new());
    input.split(",").into_iter().for_each(|x| {
        if x.contains("=") {
            let (label, value) = x.split_once("=").unwrap();
            let hash = parse(label);
            let is_it_there = map[hash]
                .iter()
                .position(|x| x.label == label)
                .unwrap_or(usize::MAX);
            let new_entry = Entry {
                label: label.to_string(),
                value: value.parse().unwrap(),
            };

            if is_it_there != usize::MAX {
                map[hash][is_it_there] = new_entry;
            } else {
                map[hash].push(new_entry);
            }
        }
        if x.contains("-") {
            let (label, _) = x.split_once("-").unwrap();
            let hash = parse(label);
            let is_it_there = map[hash]
                .iter()
                .position(|x| x.label == label)
                .unwrap_or(usize::MAX);
            if is_it_there != usize::MAX {
                map[hash].remove(is_it_there);
            }
        }
    });
    let mut sum = 0;
    for (y_idx, i) in map.iter().enumerate() {
        for (x_idx, j) in i.iter().enumerate() {
            sum += (y_idx + 1) * (x_idx+1) * j.value;
        }
    }
    println!("sum: {}", sum);
}

fn main() {
    let start = std::time::Instant::now();
    part2();
    let end = std::time::Instant::now();
    println!("{:?}", (end - start));
}
