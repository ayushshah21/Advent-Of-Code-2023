// use std::fs;

// pub fn run() {
//     let mut total_area = 0;
//     let file_contents = fs::read_to_string("./input-files/day2.txt").expect("Failed to read file");
//     //    println!("{file_contents}");
//     for line in file_contents.lines() {
//         let parts: Vec<i32> = line
//             .split('x')
//             .map(|part| part.parse::<i32>().unwrap())
//             .collect();
//         let mut length = 0;
//         let mut width = 0;
//         let mut height = 0;
//         let mut min = i32::MAX;
//         println!("INFO {:?}", parts);
//         length = parts[0] * parts[1];
//         width = parts[1] * parts[2];
//         height = parts[0] * parts[2];
//         println!("{length} {width} {height}");
        
//         if length <= width && length <= height{
//             min = length;
//         }
//         else if width <= height && width <= length{
//             min = width;
//         }
//         else if height <= length && height <= length{
//             min = height;
//         }
//         println!("MIN -->  {min}");
//         length *= 2;
//         width *= 2;
//         height *= 2;
//         total_area += length + width + height + min ;
//     }
//     println!("{total_area}");
// }
