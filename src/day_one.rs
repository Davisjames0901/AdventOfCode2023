use std::fs;
use std::io::{self, BufRead};

const LOOKUP: [(&str, i32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9)
];

pub fn run() {
    let file = fs::File::open("/home/jdavis/RustroverProjects/AdventOfCode2023/src/day_one_dat.txt")
        .expect("Should have been able to read the file");
    let lines = io::BufReader::new(file).lines();

    let mut nums = Vec::new();

    for line_read in lines {
        let mut first: i32 = -1;
        let mut last: i32 = -1;

        let line = line_read.expect("Unable to read file").to_ascii_lowercase();

        first = first_num(&line, false);
        last = first_num(&line, true);

        nums.push(first * 10 + last);
    }

    let mut total = 0;
    for num in nums {
        total += num;
    }

    print!("Total: {}", total);
}

fn first_num(input: &String, reversed: bool) -> i32 {
    let line = if reversed { input.chars().rev().collect() } else { input.to_string() };
    let mut best_string: (i32, i32) = (-1, -1);

    //gotta look for the strings :/
    for name in LOOKUP {
        let mut term = name.0.to_string();
        if (reversed) {
            term = term.chars().rev().collect();
        }

        let result = match line.find(&term) {
            Some(x) => (x as i32, name.1),
            None => (-1, -1)
        };
        if (result.0 != -1 && result.0 < best_string.0 || best_string.0 == -1) {
            best_string = result;
        }
    }

    let mut i = 0;
    for c in line.chars() {
        if (i >= best_string.0 && best_string.0 != -1) {
            break;
        }

        if (c.is_digit(10)) {
            return c.to_digit(10).unwrap() as i32;
        }
        i += 1;
    }

    return best_string.1;
}