use crate::util::*;
use std::collections::HashMap;
use itertools::Itertools;

const WIN: i32 = 6;
const TIE: i32 = 3;

pub fn day2() {
    let input = match util::read_file_str("src/days/day2/input.txt") {
        Some(x) => x,
        None => Vec::new()
    };

    let rps = HashMap::from([
        ("A", "X"),
        ("B", "Y"),
        ("C", "Z")
    ]);

    let win_map = HashMap::from([
        ("Y", "A"),
        ("X", "C"),
        ("Z", "B")
    ]);

    let score_map = HashMap::from([
        ("X", 1),
        ("Y", 2),
        ("Z", 3)
    ]);

    let mut total_score = 0;

    for x in input {
        let (elf, me) = x.split_whitespace().next_tuple().unwrap();
        //print!("{} : {}: ", elf, me);
        if win_map.get(me).unwrap() == &elf {
            total_score += WIN + score_map.get(me).unwrap();
            //println!("WIN");
        } else if rps.get(elf).unwrap() == &me {
            total_score += TIE + score_map.get(me).unwrap();
            //println!("TIE");
        } else {
            total_score += score_map.get(me).unwrap();
            //println!("LOSE");
        }
    }

    println!("{}", total_score);
}

pub fn day2_pt2() {
    let input = match util::read_file_str("src/days/day2/input.txt") {
        Some(x) => x,
        None => Vec::new()
    };

    let score_map = HashMap::from([
        ("A", 1),
        ("B", 2),
        ("C", 3)
    ]);

    let win_lose_map = HashMap::from([
        ("B", "A"),
        ("A", "C"),
        ("C", "B")
    ]);

    let mut total_score = 0;

    for x in input {
        let (elf, res) = x.split_whitespace().next_tuple().unwrap();
        //print!("{} : {}: ", elf, res);
        match res {
            "X" => { // LOSE
                let lose = win_lose_map.get(elf).unwrap();
                //println!("LOSE : {}", lose);
                total_score += score_map.get(lose).unwrap();
            },
            "Y" => { //TIE
                total_score += TIE + score_map.get(elf).unwrap();
                //println!("TIE");
            },
            "Z" => { //WIN
                let (win, _) = win_lose_map.iter().find(|(_k, v)| v == &&elf).unwrap();
                //println!("WIN : {}", win);
                total_score += WIN + score_map.get(win).unwrap();
            },
            _   => ()
        }
    }

    println!("{}", total_score);
}