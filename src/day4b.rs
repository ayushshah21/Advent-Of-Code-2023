use std::collections::HashMap;
use std::fmt::format;
use std::fs;

pub fn run() {
    let mut houses = 1;
    let file_contents =
        fs::read_to_string("./input-files/2023-day1.txt").expect("Failed to read file");
    let mut map = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);
    map.insert("four", 4);
    map.insert("five", 5);
    map.insert("six", 6);
    map.insert("seven", 7);
    map.insert("eight", 8);
    map.insert("nine", 9);

    let mut nums = Vec::<String>::new();
    let mut total = 0;
    let mut total_count = 1;

    for line in file_contents.lines() {
        let mut count = 0;
        let mut x = 0;
        let mut y = 0;
        for (index, value) in line.to_string().chars().enumerate() {
            let mut found = false;
            let substring = &line[count..];
            if (value.is_numeric() && x == 0) {
                x = value.to_digit(10).unwrap();
            }
            if value.is_numeric() {
                y = value.to_digit(10).unwrap();
                count += 1;
                continue;
            } 
            for(key, &value) in map.iter(){
                if(substring.starts_with(key)){
                    if(x == 0){
                        x = value;
                    }
                    y = value;
                    // print!("{key}");
                    // println!("Length:{:?} ", key.len());
                }
            }
            count += 1;
        }
        let num = format!("{}{}", x, y);
        let convert: i32 = num.parse().unwrap();
        println!("{total_count}: {convert}");
        total += convert;
        total_count += 1;
    }
    println!("{total}");
}
