// use std::fs;

// pub fn run() {
//     let mut ribbons = 0;
//     let file_contents = fs::read_to_string("./input-files/day2.txt").expect("Failed to read file");
//     for line in file_contents.lines() {
//         let parts: Vec<i32> = line
//             .split('x')
//             .map(|part| part.parse::<i32>().unwrap())
//             .collect();
//         let volume = parts[0] * parts[1] * parts[2];
//         let mut perimeter = 0;
//         println!("INFO {:?}", parts);
//         println!("Volume {:?}", volume);
        
//         if parts[0] <= parts[1] && parts[2] <= parts[1]{
//             perimeter = (parts[0] * 2) + (parts[2] * 2);
//         }
//         else if parts[1] <= parts[2] && parts[0] <= parts[2]{
//             perimeter = (parts[1] * 2) + (parts[0] * 2);
//         }
//         else if parts[2] <= parts[0] && parts[1] <= parts[0]{
//             perimeter = (parts[2] * 2) + (parts[1] * 2);
//         }
//         ribbons += (volume + perimeter);
//     }
//     println!("{ribbons}");
// }
