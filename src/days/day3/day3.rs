use crate::util::*;
use std::collections::{HashMap, HashSet};
use itertools::Itertools;


pub fn day3() {
    let input = match util::read_file_str("src/days/day3/input.txt") {
        Some(x) => x,
        None => Vec::new()
    };

    let mut letter_vals: Vec<(char, i32)> = ('a'..='z').zip(1..=26).collect();
    let mut uppers: Vec<(char, i32)> = ('A'..='Z').zip(27..=52).collect();

    letter_vals.append(&mut uppers);

    let chars_map: HashMap<char, i32> = HashMap::from_iter(letter_vals);

    let mut total = 0;

    for x in input {
        let (left, right) = x.split_at(x.len()/2);
        //println!("{} : {}", left, right);
        let mut left_chars: HashSet<char> = HashSet::new();
        for c in left.chars() {
            left_chars.insert(c);
        }

        for c in right.chars() {
            if left_chars.contains(&c) {
                total += chars_map.get(&c).unwrap();
                break;
            }
        }
    }

    println!("{}", total);
}

pub fn day3_pt2() {
    let input = match util::read_file_str("src/days/day3/input.txt") {
        Some(x) => x,
        None => Vec::new()
    };

    let three_grps = input.iter().tuples::<(_,_,_)>();

    let mut letter_vals: Vec<(char, i32)> = ('a'..='z').zip(1..=26).collect();
    let mut uppers: Vec<(char, i32)> = ('A'..='Z').zip(27..=52).collect();

    letter_vals.append(&mut uppers);

    let chars_map: HashMap<char, i32> = HashMap::from_iter(letter_vals);

    let mut total = 0;

    for (left, mid, right) in three_grps {
        let left_set: HashSet<char> = HashSet::from_iter(left.chars());
        let mid_set: HashSet<char> = HashSet::from_iter(mid.chars());
        let right_set: HashSet<char> = HashSet::from_iter(right.chars());

        let intersect = &(&left_set & &mid_set) & &right_set;

        let val = intersect.iter().last().unwrap();

        total += chars_map.get(val).unwrap();
    }


    println!("{total}");
}