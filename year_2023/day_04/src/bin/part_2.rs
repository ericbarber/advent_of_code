use std::fs;
use std::collections::HashSet;

fn main() {
    println!("Day 3 part 2");
    let input = fs::read_to_string("./data/input.txt").expect("Failed to read input file");
    let result = sum_gear_ratios(&input);
    println!("Sum of all gear ratios: {}", result);
}

fn sum_gear_ratios(input: &str) -> i32 {
    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut sum = 0;
    let mut counted_positions = HashSet::new();

    for (y, row) in lines.iter().enumerate() {
        for (x, &ch) in row.iter().enumerate() {
            if ch == '*' {
                let mut part_numbers = Vec::new();

                // Check all adjacent positions
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dy == 0 && dx == 0 { continue; } // Skip the gear itself
                        let adj_x = x as i32 + dx;
                        let adj_y = y as i32 + dy;

                        if counted_positions.contains(&(adj_x, adj_y)) {
                            continue; // Skip if this position is already counted
                        }

                        if let Some(number) = get_full_number_at(&lines, adj_x, adj_y) {
                            part_numbers.push(number);
                            mark_number_as_counted(&lines, adj_x, adj_y, &mut counted_positions);
                        }
                    }
                }

                // If exactly two part numbers are found, multiply them and add to sum
                if part_numbers.len() == 2 {
                    sum += part_numbers[0] * part_numbers[1];
                }
            }
        }
    }

    sum
}

fn mark_number_as_counted(lines: &[Vec<char>], x: i32, y: i32, counted: &mut HashSet<(i32, i32)>) {
    // Mark the positions of the digits of the number starting at (x, y) as counted
    if x < 0 || y < 0 || y as usize >= lines.len() || x as usize >= lines[y as usize].len() {
        return;
    }
    if !lines[y as usize][x as usize].is_digit(10) {
        return;
    }

    let mut x_pos = x;
    while x_pos < lines[y as usize].len() as i32 && lines[y as usize][x_pos as usize].is_digit(10) {
        counted.insert((x_pos, y));
        x_pos += 1;
    }
}

fn get_full_number_at(lines: &[Vec<char>], x: i32, y: i32) -> Option<i32> {
    if x < 0 || y < 0 || y as usize >= lines.len() || x as usize >= lines[y as usize].len() {
        return None;
    }
    if !lines[y as usize][x as usize].is_digit(10) {
        return None;
    }

    let mut num = String::new();
    let mut x_pos = x;

    // Move left to the start of the number
    while x_pos >= 0 && lines[y as usize][x_pos as usize].is_digit(10) {
        x_pos -= 1;
    }
    x_pos += 1; // Move back to the first digit

    // Extract the full number
    while x_pos < lines[y as usize].len() as i32 && lines[y as usize][x_pos as usize].is_digit(10) {
        num.push(lines[y as usize][x_pos as usize]);
        x_pos += 1;
    }

    Some(num.parse::<i32>().unwrap_or(0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_gear_ratios() {
        let test_input = 
            "467..114..\n\
             ...*......\n\
             ..35..633.\n\
             ......#...\n\
             617*......\n\
             .....+.58.\n\
             ..592.....\n\
             ......755.\n\
             ...$.*....\n\
             .664.598..";

        let expected_result = 467835; // Sum of gear ratios in the example
        let actual_result = sum_gear_ratios(&test_input);

        assert_eq!(actual_result, expected_result);
    }
}


// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_sum_part_numbers() {
//         let test_input = 
//             "467..114..\n\
//              ...*......\n\
//              ..35..633.\n\
//              ......#...\n\
//              617*......\n\
//              .....+.58.\n\
//              ..592.....\n\
//              ......755.\n\
//              ...$.*....\n\
//              .664.598..";
//
//         let expected_result = 4361; // Sum of part numbers in the example
//         let actual_result = sum_part_numbers(&test_input);
//
//         assert_eq!(actual_result, expected_result);
//     }
// }
//
