use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub fn read_file(path: &str) -> Option<Vec<String>> {
    read_file_str(path)
}

// Read file and return array of lines.
pub fn read_file_str(path: &str) -> Option<Vec<String>> {
    let os_path = Path::new(&path);
    let file = match File::open(path) {
        Ok(f) => f,
        Err(err) => return None,
    };
    let reader = io::BufReader::new(file).lines();
    let mut arr: Vec<String> = Vec::new();

    for line in reader {
        if let Ok(l) = line {
            arr.push(l);
        }
    }
    return Some(arr);
}

pub fn read_file_nums(path: &str) -> Option<Vec<i32>> {
    let os_path = Path::new(&path);
    let file = match File::open(path) {
        Ok(f) => f,
        Err(err) => return None,
    };
    let reader = io::BufReader::new(file).lines();
    let mut arr: Vec<i32> = Vec::new();

    for line in reader {
        if let Ok(l) = line {
            arr.push(l.trim().parse::<i32>().unwrap());
        }
    }
    return Some(arr);
}

pub fn read_groups_of_lines_as_nums(path: &str) -> Option<Vec<i32>> {
    let os_path = Path::new(&path);
    let file = match File::open(path) {
        Ok(f) => f,
        Err(err) => return None,
    };
    let reader = io::BufReader::new(file).lines();
    let mut arr: Vec<i32> = Vec::new();

    let mut total = 0;

    for line in reader {
        if let Ok(l) = line {
            if l.trim().eq("") {
                arr.push(total);
                total = 0;
                continue;
            }
            let n = l.trim().parse::<i32>().unwrap();
            total += n;
        }
    }
    return Some(arr);
}