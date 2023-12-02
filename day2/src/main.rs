use std::cmp::max;

const RED: usize = 12;
const GREEN: usize = 13;
const BLUE: usize = 14;

fn part1() {
    let input = include_str!("./input");
    let splits: Vec<_> = input
        .lines()
        .into_iter()
        .map(|line| {
            let start = line.find(":").unwrap();
            line[start + 1..]
                .split(";")
                .map(|x| x.trim().split(",").collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .collect();

    let mut sum = 0;
    for (idx, sets) in splits.iter().enumerate() {
        let mut legal = true;

        for pairs in sets {
            for pair in pairs {
                let elements: Vec<_> = pair.trim().split(" ").collect();
                println!("{:?}", elements);
                match elements[1] {
                    "red" => {
                        if elements[0].parse::<usize>().unwrap() > RED {
                            legal = false;
                            break;
                        }
                    }
                    "blue" => {
                        if elements[0].parse::<usize>().unwrap() > BLUE {
                            legal = false;
                            break;
                        }
                    }
                    "green" => {
                        if elements[0].parse::<usize>().unwrap() > GREEN {
                            legal = false;
                            break;
                        }
                    }
                    _ => panic!("not good"),
                }
            }
        }

        if legal {
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
            "red" => {
                self.red = max(self.red, value);
            }
            "blue" => {
                self.blue = max(self.blue, value);
            }
            "green" => {
                self.green = max(self.green, value);
            }
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
            let start = line.find(":").unwrap();
            line[start + 1..]
                .split(";")
                .map(|x| x.trim().split(",").collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .collect();

    let mut sum = 0;
    for sets in splits {
        sum += sets
            .iter()
            .flatten()
            .fold(Maxes::new(), |mut max, x| {
                let elements: Vec<_> = x.trim().split(" ").collect();
                max.set(&elements);
                max
            })
            .power();
    }
    println!("{sum}");
}

fn main() {
    part2();
}
