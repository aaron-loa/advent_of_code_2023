fn rotate(array: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = array.len();
    let cols = array[0].len();
    let mut rotated = vec![vec!['.'; rows]; cols];

    for original_y in 0..rows {
        for original_x in 0..cols {
            rotated[original_x][original_y] = array[original_y][original_x];
        }
    }

    rotated
}

fn process_vector(vector: &Vec<Vec<char>>) -> Option<usize> {
    for i in 0..vector.len() - 1 {
        let iter_backwards = vector[..=i].iter().rev();
        let iter_forwards = vector[i + 1..].iter();
        let mut change_this = (usize::MAX, usize::MAX);

        let is_reflection = iter_backwards
            .zip(iter_forwards)
            .enumerate()
            .try_fold(false, |mut second_chance, (main_index, (a, b))| {
                let diff = a
                    .iter()
                    .zip(b.iter())
                    .enumerate()
                    .filter(|(_, (left, right))| left != right)
                    .collect::<Vec<_>>();
                if diff.len() > 1 {
                    return None;
                } else if diff.len() == 1 && !second_chance {
                    change_this = ((main_index as i32 - i as i32).abs() as usize, diff[0].0);
                    println!("change_this: {:?}", change_this);
                    second_chance = true;
                    return Some(second_chance);
                } else {
                    return Some(second_chance);
                }
            }).unwrap_or(false);
            if is_reflection {
                return Some(i);
            }
    }
    return None;
}

fn part2() {
    let input = include_str!("./input");

    let groups = input.split("\n\n").collect::<Vec<&str>>();
    let mut sum = 0;
    let mut i = 0;
    for group in groups {
        println!("group: {}", i);
        i += 1;
        let mut vector = group
            .split("\n")
            .map(|x| x.chars().collect::<Vec<_>>())
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>();

        let res = process_vector(&vector);
        let mut found = false;
        let mut reflection_y = 0;
        if let Some(line_idx) = res {
            // swap happened
            reflection_y = line_idx + 1;
            found = true;
        }

        let mut reflection_x = 0;
        if !found {
            println!("rotating");
            let rotated = rotate(&vector);
            let res = process_vector(&rotated);
            if let Some(line_idx) = res {
                // swap happened
                reflection_x = line_idx + 1;
            }
        }
        println!(
            "reflection_x: {}, reflection_y: {}",
            reflection_x, reflection_y
        );
        sum += reflection_x + reflection_y * 100;
    }
    println!("sum: {}", sum);
}

fn part1() {
    let input = include_str!("./input");

    let groups = input.split("\n\n").collect::<Vec<&str>>();
    let mut sum = 0;
    for group in groups {
        let vector = group
            .split("\n")
            .map(|x| x.chars().collect::<Vec<_>>())
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>();
        let mut reflection_y = 0;
        for i in 0..vector.len() - 1 {
            let iter_backwards = vector[..=i].iter().rev();
            let iter_forwards = vector[i + 1..].iter();
            let is_reflection = iter_backwards.zip(iter_forwards).all(|(a, b)| a == b);
            if is_reflection {
                reflection_y = i + 1;
                break;
            }
        }

        let rotated = rotate(&vector);
        let mut reflection_x = 0;
        for i in 0..rotated.len() - 1 {
            let iter_backwards = rotated[..=i].iter().rev();
            let iter_forwards = rotated[i + 1..].iter();
            let is_reflection = iter_backwards.zip(iter_forwards).all(|(a, b)| a == b);
            if is_reflection {
                reflection_x = i + 1;
                break;
            }
        }
        sum += reflection_x + reflection_y * 100;
    }
    println!("sum: {}", sum);
}

fn main() {
    let start = std::time::Instant::now();
    part2();
    let end = std::time::Instant::now();
    println!("{:?}", end - start);
}
