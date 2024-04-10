use std::collections::HashMap;
use std::fmt::format;
use std::fs;

pub fn run() {
    let file_contents =
        fs::read_to_string("./input-files/2023-day2.txt").expect("Failed to read file");

    let mut nums = Vec::<String>::new();
    let mut total = 0;
    let mut count = 1;
    for line in file_contents.lines(){
        let mut is_possible = true;
        let id_separator:Vec<&str> = line.split(":").collect();
        let id_type: &str = id_separator[0];
        let id_str :Vec<&str>= id_type.split_whitespace().collect();
        let id: i32 = id_str[1].parse().unwrap();
        let game_contents = id_separator[1];
        let sub_games:Vec<&str> = game_contents.split(";").collect();
        let sub_games_length = sub_games.len();
        // println!("Game {count}:");
        for game in sub_games{
            let game_vals:Vec<&str> = game.split(",").collect();
            for single_game in game_vals{
                let sub_game_contents:Vec<&str> = single_game.split_whitespace().collect();
                let num: i32 = sub_game_contents[0].parse().unwrap();
                let color: &str = sub_game_contents[1];
                if color == "green"{
                    if num > 13{
                        is_possible = false;
                    }
                }
                else if color == "red"{
                    if num > 12{
                        is_possible = false;
                    }
                }
                else if color == "blue"{
                    if num > 14{
                        is_possible = false;
                    }
                }
            }
        }
        // println!("Game {count}: {is_possible}");
        if is_possible{
            total += id;
        }
        else{
            println!("Game {count}")
        }
        count += 1;
    }
    println!("Answer: {total}");
}


