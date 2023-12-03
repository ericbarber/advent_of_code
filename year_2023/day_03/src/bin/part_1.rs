use std::fs;

fn main() {
    println!("Day 3 part 1");
    let input = fs::read_to_string("./data/input.txt").expect("Failed to read input file");
    let sum = sum_part_numbers(&input);
    println!("Sum of part numbers: {}", sum);
}

fn sum_part_numbers(input: &str) -> i32 {
    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut sum = 0;
    let mut counted = std::collections::HashSet::new();

    for (y, row) in lines.iter().enumerate() {
        for (x, &ch) in row.iter().enumerate() {
            if !ch.is_digit(10) && ch != '.' {
                // Check all adjacent positions
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dy == 0 && dx == 0 { continue; } // Skip the symbol itself
                        let adj_x = x as i32 + dx;
                        let adj_y = y as i32 + dy;
                        if adj_x >= 0 && adj_y >= 0 && adj_y < lines.len() as i32 && adj_x < lines[adj_y as usize].len() as i32 {
                            let adj_ch = lines[adj_y as usize][adj_x as usize];
                            if adj_ch.is_digit(10) && !counted.contains(&(adj_x, adj_y)) {
                                // Extract the full number
                                let number = extract_number(&lines, adj_x, adj_y);
                                sum += number;
                                // Mark all digits of this number as counted
                                mark_number_as_counted(&lines, adj_x, adj_y, &mut counted);
                            }
                        }
                    }
                }
            }
        }
    }

    sum
}

fn extract_number(lines: &[Vec<char>], x: i32, y: i32) -> i32 {
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

    num.parse::<i32>().unwrap_or(0)
}

fn mark_number_as_counted(lines: &[Vec<char>], x: i32, y: i32, counted: &mut std::collections::HashSet<(i32, i32)>) {
    let mut x_pos = x;

    // Move left to the start of the number
    while x_pos >= 0 && lines[y as usize][x_pos as usize].is_digit(10) {
        x_pos -= 1;
    }
    x_pos += 1; // Move back to the first digit

    // Mark all digits of this number as counted
    while x_pos < lines[y as usize].len() as i32 && lines[y as usize][x_pos as usize].is_digit(10) {
        counted.insert((x_pos, y));
        x_pos += 1;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_part_numbers() {
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

        let expected_result = 4361; // Sum of part numbers in the example
        let actual_result = sum_part_numbers(&test_input);

        assert_eq!(actual_result, expected_result);
    }
}

