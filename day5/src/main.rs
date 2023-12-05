use std::ops::{Deref, Range, RangeBounds};

fn part2() {
    let input = include_str!("./input");

    let numbers: Vec<_> = input
        .lines()
        .into_iter()
        .nth(0)
        .unwrap()
        .split_whitespace()
        .filter_map(|x| match x.parse::<usize>() {
            Ok(x) => return Some(x),
            _ => return None,
        })
        .collect();

    let numbers: Vec<_> = numbers
        .chunks(2)
        .map(|element| (element[0]..(element[0] + element[1])))
        .collect();
    let lines_len = input.lines().count() - 1;
    for number_chunk in numbers {
        let numbers: Vec<_> = number_chunk.collect();
        let sum: usize = input
            .lines()
            .into_iter()
            .enumerate()
            .skip(2)
            .fold(
                (numbers, Vec::<(Range<usize>, Range<usize>)>::new()),
                |(mut state, mut ranges), (idx, line)| {
                    if line == "" {
                        return (state, ranges);
                    }
                    if !line.contains(":") {
                        let parsed: Vec<_> = line
                            .split_whitespace()
                            .map(|x| x.parse::<usize>().unwrap())
                            .collect();

                        let destination = parsed[0]..(parsed[0] + parsed[2]);
                        let source = parsed[1]..(parsed[1] + parsed[2]);
                        ranges.push((destination, source));
                    }
                    if line.contains(":") || lines_len <= idx {
                        if ranges.len() != 0 {
                            state.iter_mut().enumerate().for_each(|(_, position)| {
                                for (destination, source) in &ranges {
                                    if source.contains(position) {
                                        let new_position = *position - source.start;
                                        // println!(
                                        //     "[{}] Moving {} to {}",
                                        //     index,
                                        //     position,
                                        //     destination.start + new_position
                                        // );
                                        *position = destination.start + new_position;
                                        break;
                                    }
                                }
                            });
                            // println!("{:?}", ranges);
                            // println!("{:?}", state);
                            ranges = vec![];
                        }
                        return (state, ranges);
                    }
                    return (state, ranges);
                },
            )
            .0
            .iter()
            .min()
            .unwrap()
            .clone();

        println!("{:?}", sum);
        // input.lines().into_iter().map(|line|{
        //     match line.split_once(":") {
        //         None
        //     }
        // })
    }
}
fn part1() {
    let input = include_str!("./input");

    let numbers: Vec<_> = input
        .lines()
        .into_iter()
        .nth(0)
        .unwrap()
        .split_whitespace()
        .filter_map(|x| match x.parse::<usize>() {
            Ok(x) => return Some(x),
            _ => return None,
        })
        .collect();

    let lines_len = input.lines().count() - 1;

    let sum: usize = input
        .lines()
        .into_iter()
        .enumerate()
        .skip(2)
        .fold(
            (numbers, Vec::<(Range<usize>, Range<usize>)>::new()),
            |(mut state, mut ranges), (idx, line)| {
                if line == "" {
                    return (state, ranges);
                }
                if !line.contains(":") {
                    let parsed: Vec<_> = line
                        .split_whitespace()
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect();

                    let destination = parsed[0]..(parsed[0] + parsed[2]);
                    let source = parsed[1]..(parsed[1] + parsed[2]);
                    ranges.push((destination, source));
                }
                if line.contains(":") || lines_len <= idx {
                    if ranges.len() != 0 {
                        state.iter_mut().enumerate().for_each(|(index, position)| {
                            for (destination, source) in &ranges {
                                if source.contains(position) {
                                    let new_position = *position - source.start;
                                    // println!(
                                    //     "[{}] Moving {} to {}",
                                    //     index,
                                    //     position,
                                    //     destination.start + new_position
                                    // );
                                    *position = destination.start + new_position;
                                    break;
                                }
                            }
                        });
                        ranges = vec![];
                    }
                    return (state, ranges);
                }
                return (state, ranges);
            },
        )
        .0
        .iter()
        .min()
        .unwrap()
        .clone();

    println!("{:?}", sum);
    // input.lines().into_iter().map(|line|{
    //     match line.split_once(":") {
    //         None
    //     }
    // })
}

fn main() {
    let start = std::time::Instant::now();
    part2();
    let end = std::time::Instant::now();
    println!("{:?}", (end - start));
}
