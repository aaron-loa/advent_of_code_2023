use std::ops::Deref;

fn walk(slice: &[char], position: usize, sizes: &Vec<usize>, counter: &mut usize) -> bool {
    if position == sizes.len() {
        println!("{} found", "\t".repeat(position));
        *counter += 1;
        return true;
    }

    if sizes[position] == *(sizes.get(position + 1).unwrap_or(&1000)) {
        let pos = slice
            .windows(sizes[position])
            .position(|w| w.iter().all(|c| *c == '#'));
        if pos.is_some() {
            return walk(&slice[pos.unwrap()..], position + 1, &sizes, counter);
        }
    }


    println!(
        "{}{:?} <slice postion {} pos {:?}",
        "\t".repeat(position),
        slice,
        sizes[position],
        position,
    );

    for (idx, window) in slice.windows(sizes[position]).enumerate() {
        println!("{}{:?} window", "\t".repeat(position), window);
        if window[0] == '#' || window[window.len() - 1] == '#' {
            continue;
        }

        if window[1..window.len() - 1].iter().any(|c| *c == '.') {
            continue;
        }

        if window.iter().all(|c| *c == '#') {
            return false;
        }

        let is_all_hash = window[1..window.len() - 1].iter().all(|c| *c == '#');
        if is_all_hash {
            return walk(
                &slice[idx + sizes[position] - 1..],
                position + 1,
                &sizes,
                counter,
            );
        }
        if window[1..window.len() - 1].iter().any(|c| *c == '#') {
            return walk(
                &slice[idx + sizes[position] - 1..],
                position + 1,
                &sizes,
                counter,
            );
        }
        walk(
            &slice[idx + sizes[position] - 1..],
            position + 1,
            &sizes,
            counter,
        );
    }
    return true;
}

fn part1() {
    let input = include_str!("./input");
    let sum = input
        .lines()
        .map(|line| {
            let (path, numbers) = line.split_once(" ").unwrap();
            let mut numbers = numbers
                .split(",")
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            numbers.iter_mut().for_each(|n| *n += 2);
            let mut path: Vec<_> = path.chars().collect();
            path.push('.');
            path.insert(0, '.');
            let mut counter = 0;
            if path.iter().filter(|c| **c == '#').count()
                == numbers.iter().map(|x| x - 2).sum::<usize>()
            {
                return 1;
            }
            walk(&path, 0, &numbers, &mut counter);
            println!("{}", counter);
            counter
        })
        .sum::<usize>();
    println!("{}", sum)
}

fn main() {
    let start = std::time::Instant::now();
    part1();
    let end = std::time::Instant::now();
    println!("{:?}", end - start);
}
