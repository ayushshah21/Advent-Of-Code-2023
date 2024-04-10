use std::fs;

pub fn run() {
    let file_contents =
        fs::read_to_string("./input-files/2023-day2.txt").expect("Failed to read file");

    let mut total = 0;
    let mut count = 1;
    for line in file_contents.lines() {
        let id_separator: Vec<&str> = line.split(":").collect();
        let id_type: &str = id_separator[0];
        let id_str: Vec<&str> = id_type.split_whitespace().collect();
        let game_contents = id_separator[1];
        let sub_games: Vec<&str> = game_contents.split(";").collect();
        let sub_games_length = sub_games.len();
        println!("Game {count}:");
        let mut red_max = 1;
        let mut green_max = 1;
        let mut blue_max = 1;
        for game in sub_games {
            // println!("{game}");
            let game_vals: Vec<&str> = game.split(",").collect();
            for single_game in game_vals {
                let sub_game_contents: Vec<&str> = single_game.split_whitespace().collect();
                let num: i32 = sub_game_contents[0].parse().unwrap();
                let color: &str = sub_game_contents[1];
                if color == "green" {
                    if num > red_max {
                        red_max = num;
                    }
                } else if color == "red" {
                    if num > green_max {
                        green_max = num;
                    }
                } else if color == "blue" {
                    if num > blue_max {
                        blue_max = num;
                    }
                }
            }
        }
        println!("Red max: {red_max}, Green max: {green_max}, Blue max: {blue_max}");
        let combined_max = red_max * green_max * blue_max;
        total += combined_max;
        count += 1;
    }
    println!("Total: {total}");
}
