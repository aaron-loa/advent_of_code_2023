use std::{cmp::max, collections::HashMap, io::ErrorKind};

fn part1() {
    let input = include_str!("./input");
    let splits: Vec<_> = input
        .lines()
        .into_iter()
        .map(|line| {
            let (_, rest) = line.split_once(":").expect("not good");
            rest.split(";")
                .map(|x| x.trim().split(",").collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .collect();

    let mut sum = 0;
    let lut = HashMap::from([("green", 13usize), ("red", 12), ("blue", 14)]);

    for (idx, sets) in splits.iter().enumerate() {
        if sets.iter().flatten().all(|x| {
            let (value, color) = x.trim().split_once(" ").expect("not good");
            if value.parse::<usize>().unwrap() > *lut.get(color).unwrap() {
                return false;
            }
            return true;
        }) {
            sum += idx + 1;
        }
    }
    println!("{sum}");
}

struct Maxes {
    green: usize,
    red: usize,
    blue: usize,
}

impl Maxes {
    fn new() -> Self {
        Self {
            blue: 0,
            green: 0,
            red: 0,
        }
    }

    fn set(&mut self, elements: &Vec<&str>) {
        let value = elements[0].parse::<usize>().unwrap();
        match elements[1] {
            "red" => self.red = max(self.red, value),
            "blue" => self.blue = max(self.blue, value),
            "green" => self.green = max(self.green, value),
            _ => panic!("not good"),
        }
    }

    fn power(&self) -> usize {
        return self.blue * self.green * self.red;
    }
}

fn part2() {
    let input = include_str!("./input");
    let splits: Vec<_> = input
        .lines()
        .into_iter()
        .map(|line| {
            let (_, rest) = line.split_once(":").expect("not good");
            rest.split(";")
                .map(|x| x.trim().split(",").collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .collect();

    let sum: usize = splits
        .iter()
        .map(|x| {
            x.iter()
                .flatten()
                .fold(Maxes::new(), |mut max, x| {
                    let elements: Vec<_> = x.trim().split(" ").collect();
                    max.set(&elements);
                    max
                })
                .power()
        })
        .sum();
    println!("{sum}");
}

fn main() {
    let start = std::time::Instant::now();
    part1();
    let end = std::time::Instant::now();
    println!("Time: {:?}", end - start);
}
