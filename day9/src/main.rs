fn process_numbers_part2(numbers: Vec<i32>) -> i32 {
    let mut stack = vec![numbers];

    while !stack[stack.len() - 1].iter().all(|x| *x == 0) {
        stack.push(
            stack[stack.len() - 1]
                .windows(2)
                .into_iter()
                .map(|x| x[0] - x[1])
                .collect(),
        );
    }
    let mut start = 0;
    loop {
        match stack.pop() {
            Some(x) => {
                start = x[x.len() - 1] + start *-1;
            } 
            None => break,
        }
    }
    
    return start;
}

fn part2() {
    let input = include_str!("./input");
    let res = input
        .lines()
        .map(|line| {
            let mut numbers = line
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            numbers.reverse();
            return process_numbers_part2(numbers);
        })
        .sum::<i32>();
    println!("{}", res);
}
fn process_numbers(numbers: Vec<i32>) -> i32 {
    let mut stack = vec![numbers];
    while !stack[stack.len() - 1].iter().all(|x| *x == 0) {
        stack.push(
            stack[stack.len() - 1]
                .windows(2)
                .into_iter()
                .map(|x| x[1] - x[0])
                .collect(),
        );
    }
    let mut start = 0;
    loop {
        match stack.pop() {
            Some(x) => {
                start = x[x.len() - 1] + start;
            }
            None => break,
        }
    }
    return start;
}

fn part1() {
    let input = include_str!("./input");
    let res = input
        .lines()
        .map(|line| {
            let numbers = line
                .split(" ")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            return process_numbers(numbers);
        })
        .sum::<i32>();
    println!("{}", res);
}

fn main() {
    let start = std::time::Instant::now();
    part2();
    let end = std::time::Instant::now();
    println!("{:?}", end - start);
}
