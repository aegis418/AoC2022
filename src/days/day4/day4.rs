use crate::util::*;
use itertools::Itertools;
use std::collections::HashSet;

pub fn day4() {
    let input = match util::read_file_str("src/days/day4/input.txt") {
        Some(x) => x,
        None => Vec::new()
    };

    let mut total = 0;

    for pair in input {
        let (left, right) = pair.split_once(',').unwrap();
        let (lstart, lend): (i32, i32) = left.split('-').map(|x| x.parse::<i32>().unwrap()).collect_tuple().unwrap();
        let (rstart, rend): (i32, i32) = right.split('-').map(|x| x.parse::<i32>().unwrap()).collect_tuple().unwrap();

        if (lstart >= rstart && lend <= rend) || (rstart >= lstart && rend <= lend) {
            total += 1;
        }
    }

    println!("{total}");
}

pub fn day4_pt2() {
    let input = match util::read_file_str("src/days/day4/input.txt") {
        Some(x) => x,
        None => Vec::new()
    };

    let mut total = 0;

    for pair in input {
        let (left, right) = pair.split_once(',').unwrap();
        let (lstart, lend): (i32, i32) = left.split('-').map(|x| x.parse::<i32>().unwrap()).collect_tuple().unwrap();
        let (rstart, rend): (i32, i32) = right.split('-').map(|x| x.parse::<i32>().unwrap()).collect_tuple().unwrap();

        let lrange: HashSet<i32> = HashSet::from_iter(lstart..=lend);
        let rrange: HashSet<i32> = HashSet::from_iter(rstart..=rend);

        let intersect = lrange.intersection(&rrange).collect::<HashSet<_>>();

        if !intersect.is_empty() {
            total += 1;
        }
    }

    println!("{total}");
}