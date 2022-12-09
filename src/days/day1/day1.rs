use crate::util::*;

pub fn day_1() {
    let input = match util::read_file_str("src/days/day1/input.txt") {
        Some(x) => x,
        None => Vec::new()
    };

    let mut max = 0;
    let mut cur = 0;

    for x in input {
        if x.eq("") {
            if cur > max {
                max = cur;
            }
            cur = 0;
            continue;
        }

        cur += x.parse::<u32>().unwrap();
    }

    println!("{:?}", max);
}

pub fn day_1_pt2() {
    let mut input = match util::read_groups_of_lines_as_nums("src/days/day1/input.txt") {
        Some(x) => x,
        None => Vec::new()
    };
    
    input.sort();
    input.reverse();
    let total: i32 = input.into_iter().take(3).sum();

    println!("{}", total);
}