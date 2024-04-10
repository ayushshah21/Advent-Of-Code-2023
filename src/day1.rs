// use std::fs;

// pub fn run() {
//     let mut floor = 0;
//     let file_contents = fs::read_to_string("./input-files/day1.txt").expect("Failed to read file");
//     // println!("info.txt context =\n{:?}", file_contents);
//     for (index, value) in file_contents.chars().enumerate() {
//         if value == '(' {
//             floor += 1;
//         }
//         if value == ')' {
//             floor -= 1;
//         }
//         if floor == -1{
//             println!("{index}");
//             break;
//         }
//     }
// }
