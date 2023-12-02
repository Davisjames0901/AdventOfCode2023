use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead};

pub fn part1() {
    let file = fs::File::open("/home/jdavis/RustroverProjects/AdventOfCode2023/src/day_two_dat.txt")
        .expect("Should have been able to read the file");
    let lines = io::BufReader::new(file).lines();

    let rules = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14)
    ]);
    let mut id_total = 0;

    'game: for line_read in lines {
        let line = line_read.expect("Unable to read file");

        let colon_tokens = line.split(":").collect::<Vec<&str>>();
        let game_title = &colon_tokens[0];

        let game_tokens = colon_tokens[1].trim().split(";").collect::<Vec<&str>>();
        let game_id = game_title.split(" ").collect::<Vec<&str>>()[1];
        println!("Game {}", game_id);

        for round in game_tokens {
            let round_tokens = round.split(",");
            for block in round_tokens {
                let block_tokens = block.trim().split(" ").collect::<Vec<&str>>();
                let block_count = &block_tokens[0].parse::<i32>().unwrap();
                let rule_value = rules.get(block_tokens[1]).unwrap();
                if(block_count > rule_value) {
                    continue 'game;
                }
            }
        }
        id_total += game_id.parse::<i32>().unwrap();
    }

    println!("Total: {}", id_total);
}

pub fn part2() {
    let file = fs::File::open("/home/jdavis/RustroverProjects/AdventOfCode2023/src/day_two_dat.txt")
        .expect("Should have been able to read the file");
    let lines = io::BufReader::new(file).lines();

    let mut id_total = 0;

    'game: for line_read in lines {
        let line = line_read.expect("Unable to read file");

        let mut game_totals = HashMap::from([
            ("red", 0),
            ("green", 0),
            ("blue", 0)
        ]);
        let colon_tokens = line.split(":").collect::<Vec<&str>>();
        let game_title = &colon_tokens[0];

        let game_tokens = colon_tokens[1].trim().split(";").collect::<Vec<&str>>();
        let game_id = game_title.split(" ").collect::<Vec<&str>>()[1];
        println!("Game {}", game_id);

        for round in game_tokens {
            let round_tokens = round.split(",");
            for block in round_tokens {
                let block_tokens = block.trim().split(" ").collect::<Vec<&str>>();
                let block_count = block_tokens[0].parse::<i32>().unwrap();
                let best_value = game_totals.get(block_tokens[1]).unwrap();
                if(&block_count > best_value) {
                    (*game_totals.get_mut(block_tokens[1]).unwrap()) = block_count;
                }
            }
        }
        let red = game_totals.get("red").unwrap();
        let green = game_totals.get("green").unwrap();
        let blue = game_totals.get("blue").unwrap();

        id_total += red * green * blue;
    }

    println!("Total: {}", id_total);
}