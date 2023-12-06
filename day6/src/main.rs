fn part2fast() {
    let time: usize = 40829166;
    let distance: usize = 277133813491063;

    let x = 0usize..time;
    let low = x
        .clone()
        .into_iter()
        .position(|speed| {
            let time_left = time - speed;
            let total = time_left * speed;
            if distance < total {
                return true;
            }
            return false;
        })
        .unwrap();

    let high = time
        - x.into_iter()
            .rev()
            .position(|speed| {
                let time_left = time - speed;
                let total = time_left * speed;
                if distance < total {
                    return true;
                }
                return false;
            })
            .unwrap();

    println!("{}", high - low);
}

fn part2_copy_paste() {
    let time: usize = 40829166;
    let distance: usize = 277133813491063;

    let x = 0usize..time;
    let numbers = x
        .into_iter()
        .filter_map(|speed| {
            let time_left = time - speed;
            let total = time_left * speed;
            if distance < total {
                return Some(speed);
            }
            return None;
        })
        .count();

    println!("\n{numbers}");
}

fn part1() {
    let times: Vec<usize> = vec![40, 82, 91, 66];
    let distance: Vec<usize> = vec![277, 1338, 1349, 1063];

    // let times: Vec<usize> = vec![7, 15, 30];
    // let distance: Vec<usize> = vec![9, 40, 200];

    let product: usize = times
        .iter()
        .zip(distance.iter())
        .map(|(time, distance)| {
            let x = 0usize..*time;
            x.into_iter()
                .filter_map(|speed| {
                    let time_left = time - speed;
                    let total = time_left * speed;
                    if *distance < total {
                        return Some(speed);
                    }
                    return None;
                })
                .count()
        })
        .product();
    println!("\n{product}");
}

fn part1_fast() {
    let times: Vec<usize> = vec![40, 82, 91, 66];
    let distance: Vec<usize> = vec![277, 1338, 1349, 1063];

    let product: usize = times
        .iter()
        .zip(distance.iter())
        .map(|(time, distance)| {
            let x = 0usize..*time;
            let low = x
                .clone()
                .into_iter()
                .position(|speed| {
                    let time_left = time - speed;
                    let total = time_left * speed;
                    if *distance < total {
                        return true;
                    }
                    return false;
                })
                .unwrap();

            let high = time
                - x.into_iter()
                    .rev()
                    .position(|speed| {
                        let time_left = time - speed;
                        let total = time_left * speed;
                        if *distance < total {
                            return true;
                        }
                        return false;
                    })
                    .unwrap();
            high-low
        })
        .product();
    println!("\n{product}");
}

fn main() {
    let start = std::time::Instant::now();
    part1();
    let end = std::time::Instant::now();
    println!("part1: {:?}", (end - start));

    let start = std::time::Instant::now();
    part1_fast();
    let end = std::time::Instant::now();
    println!("part1_fast: {:?}", (end - start));


    let start = std::time::Instant::now();
    part2_copy_paste();
    let end = std::time::Instant::now();
    println!("part2_copy_paste: {:?}", (end - start));

    let start = std::time::Instant::now();
    part2fast();
    let end = std::time::Instant::now();
    println!("part2fast: {:?}", (end - start));

}
