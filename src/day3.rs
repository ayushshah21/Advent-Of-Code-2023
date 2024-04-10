use std::fmt::format;
use std::fs;

pub fn run() {
    let mut houses = 1;
    let file_contents = fs::read_to_string("./input-files/day3.txt").expect("Failed to read file");
    let mut nums = Vec::<String>::new();
    let mut x = 0;
    let mut y = 0;

    for(_index, value) in file_contents.chars().enumerate(){
        if value == '^'{
            y += 1;
        }
        else if value == 'v'{
            y -= 1;
        }
        else if value == '<'{
            x -= 1;
        }
        else if value == '>'{
            x += 1;
        }
        let exists = format!("{}{}", x, y);
        if !nums.contains(&exists) {
            houses += 1;
            nums.push(exists);
        }
    }
    println!("{houses}");

}
