fn extract_numbers(numbers: &str) -> Vec<usize> {
    numbers
        .split_whitespace()
        .map(|x| {
            x.parse().unwrap()
        })
        .collect()
}

fn part2() {
    let input = include_str!("./input");
    let mut map = vec![1usize; input.lines().count()];
    input
        .lines()
        .into_iter()
        .enumerate()
        .for_each(|(idx, line)| {
            let (_, numbers) = line.split_once(":").unwrap();
            let (card_value, numbers) = numbers.split_once("|").unwrap();
            let lut = extract_numbers(card_value);
            let numbers = extract_numbers(numbers);
            let contains = numbers.iter().filter(|x| lut.contains(x)).count();
            let current = map[idx].clone();
            map[idx+1..=idx+contains]
                .iter_mut()
                .for_each(|x| *x += current)
        });
    println!("{}", map.iter().sum::<usize>());
}

fn part1() {
    let input = include_str!("./input");

    let result: usize = input
        .lines()
        .into_iter()
        .map(|line| {
            let (_, numbers) = line.split_once(":").unwrap();
            let (card_value, numbers) = numbers.split_once("|").unwrap();
            let lut = extract_numbers(card_value);
            let numbers = extract_numbers(numbers);
            let contains = numbers.iter().filter(|x| lut.contains(x)).count();
            if contains == 0 {
                return 0;
            }
            return 2usize.pow((contains - 1) as u32);
        })
        .sum();
    println!("{result}");
}

fn main() {
    let now = std::time::Instant::now();
    part2();
    let end = std::time::Instant::now();
    println!("{:?}", end - now);
}
