use std::fs;
use std::hash::{Hash};
use std::collections::HashMap;
use std::io::{self, BufRead};

pub fn part1() {
    let file = fs::File::open("/home/jdavis/RustroverProjects/AdventOfCode2023/src/day_three_dat.txt")
        .expect("Should have been able to read the file");
    let lines = io::BufReader::new(file).lines().map(|x| x.unwrap().chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    let mut total: i32 = 0;
    let mut part_numbers :HashMap<Location, i32> = HashMap::new();
    let mut number = Vec::new();
    let mut gear_symbol = None;

    let mut line_index = 0;

    for line in &lines {
        let mut char_index = 0;
        for char in line {
            if(char.is_digit(10)) {
                number.push(char);
                if(gear_symbol.is_none()) {
                    gear_symbol = get_adjacent_star(&lines, line_index, char_index);
                }
            } else if(!number.is_empty()) {
                if(gear_symbol.is_some()) {
                    let part_num = number.into_iter().collect::<String>().parse::<i32>().unwrap();
                    let sym = gear_symbol.unwrap();
                    let existing_part = part_numbers.get(&sym);
                    if(existing_part.is_some()) {
                        total += part_num * existing_part.unwrap();
                    }
                    else if (existing_part.is_none()) {
                        part_numbers.insert(sym, part_num);
                    }
                }
                number = Vec::new();
                gear_symbol = None;
            }

            char_index += 1;
        }
        line_index += 1;
    }

    print!("total: {}", total);
}

fn get_adjacent_star(file: &Vec<Vec<char>>, line_index: usize, char_index: usize) -> Option<Location> {
    let center = match file.get(line_index) {
        Some(line) => check_around(line, char_index, line_index),
        None => None
    };
    if(center.is_some()) {
        return center;
    }

    let below = match file.get(line_index +1) {
        Some(line) => check_around(line, char_index, line_index+1),
        None => None
    };
    if(below.is_some()) {
        return below;
    }

    let above = match file.get(safe_add(line_index, -1)) {
        Some(line) => check_around(line, char_index, safe_add(line_index, -1)),
        None => None
    };
    return above;
}

fn check_around(line: &Vec<char>, char_index: usize, line_index: usize) -> Option<Location> {
    let left = match line.get(safe_add(char_index, -1)) {
        Some(c) => if(c == &'*') { Some(Location { x: char_index-1, y: line_index })} else {None},
        None => None
    };
    if(left.is_some()) {
        return left;
    }

    let center = match line.get(char_index) {
        Some(c) => if(c == &'*') { Some(Location { x: char_index, y: line_index })} else {None},
        None => None
    };
    if(center.is_some()) {
        return center;
    }
    let right = match line.get(char_index +1) {
        Some(c) => if(c == &'*') { Some(Location { x: char_index+1, y: line_index })} else {None},
        None => None
    };
    return right;
}

fn safe_add(index: usize, modifier: i32) -> usize {
    let mut i_index = index as i32;
    i_index += modifier;
    if(i_index.is_negative()) {
        return 0;
    }
    else {
        return i_index as usize;
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Location {
    x: usize,
    y: usize
}