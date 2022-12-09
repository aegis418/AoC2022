mod util;
mod days;

use std::io;
use std::io::Write;
use crate::days::days::*;

fn main() {
    println!("AoC Runner");

    print!("Day to run: ");
    let _ = io::stdout().flush();
    let mut day = String::new();
    io::stdin().read_line(&mut day);
    println!("{}", format!("Running day {}", day));
    let day_num = match day.trim().parse::<i32>() {
        Ok(n) => n,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    match run_day(Day::from(day_num)) {
        Ok(_) => println!("Done"),
        Err(e) => println!("{}", e),
    };
}
