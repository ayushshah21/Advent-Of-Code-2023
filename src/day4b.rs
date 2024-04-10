use std::fmt::format;
use std::fs;

pub fn run() {
    let mut houses = 1;
    let file_contents = fs::read_to_string("./input-files/2023-day1.txt").expect("Failed to read file");
    let mut nums = Vec::<String>::new();
    let mut total = 0;
    

    for line in file_contents.lines(){
        let mut x = 0;
        let mut y = 0;
       for(index, value) in line.to_string().chars().enumerate(){
        if value.is_numeric() && x == 0{
            x = value.to_digit(10).unwrap();
        }
        if value.is_numeric(){
            y = value.to_digit(10).unwrap();
        }
    }
    let num = format!("{}{}", x, y);
    let convert: i32 = num.parse().unwrap();
    println!("{convert}");
    total += convert;
}
println!("{total}");
}
