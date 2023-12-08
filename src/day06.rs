fn count_strategy(time: isize, record: isize) -> isize {
    let (is_even, max) = if time % 2 == 0 {
        (true, (time / 2).pow(2))
    } else {
        (false, time / 2 * (time / 2 + 1))
    };

    let mut count = 0;
    let mut race_diff = 0;
    let record_diff = max - record;
    let mut incr = if is_even { 1 } else { 0 };

    race_diff += incr;
    while race_diff < record_diff {
        incr += 2;
        race_diff += incr;
        count += 1;
    }

    if is_even {
        count * 2 + 1
    } else {
        count * 2
    }
}

pub fn day06() {
    println!("\nDay 06");
    let race = vec![(60, 601), (80, 1163), (86, 1559), (76, 1300)];

    let part01 = race.iter().fold(1, |acc, x| acc * count_strategy(x.0, x.1));

    let part02 = count_strategy(60808676, 601116315591300);

    println!("Part 1 - No. winning strategies: {}", part01);
    println!("Part 2 - No. of winning strategies: {}", part02);
}
