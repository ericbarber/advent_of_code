use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("Day 2 part 1");
    let input = read_lines("./data/input.txt").expect("Failed to read input file");
    let result = part_process(&input).expect("Failed to process input");
    println!("Result: {}", result);
}

fn part_process(input: &[String]) -> Result<u32, &'static str> {
    let mut sum = 0;

    for (index, line) in input.iter().enumerate() {
        if !process_game_line(line) {
            if let Some((game_id, _)) = line.split_once(':') {
                sum += index as u32 + 1; // Uncomment if Game numbers are 1-indexed

                println!("Game ID: {}, Index: {}, Sum: {}", game_id.trim(), index, sum);
            } else {
                println!("No colon found in line: {}", line);
            }
        }
    }

    Ok(sum)
}

fn process_game_line(line: &str) -> bool {
    let max_cubes = [("red", 12), ("green", 13), ("blue", 14)].iter().cloned().collect::<std::collections::HashMap<_, _>>();
    let plays = line.split(';');
    for play in plays {
        let cubes = play.split(',').map(|s| s.trim());
        let mut counts = std::collections::HashMap::new();
        for cube in cubes {
            let parts: Vec<&str> = cube.split_whitespace().collect();
            if parts.len() != 2 {
                continue; // Skip if not a valid cube description
            }
            let count: u32 = parts[0].parse().unwrap_or(0);
            let color = parts[1];
            *counts.entry(color).or_insert(0) += count;
        }
        for (color, &count) in &counts {

            if count > *max_cubes.get(color).unwrap_or(&u32::MAX) {
                // println!("{} : {} \t {}", count, *max_cubes.get(color).unwrap_or(&u32::MAX), color);
                return true;
            }
        }
    }
    // If not rounds exceeded cube limits, return false.
    false
}

fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();
    lines.collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_two() {
        let test_input = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string(),
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string(),
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string(),
            // testing if the game will catch limits
            "Game 6: 12 red, 14 blue, 13 green; 14 blue, 12 red, 13 green".to_string(),
        ];

        let expected_result = 1 + 2 + 5 + 6; // Sum of game numbers that did not exceed the limit
        let actual_result = part_process(&test_input).expect("Failed to process test input");

        assert_eq!(actual_result, expected_result);
    }
}
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_part_one() {
//         let test_input = read_lines("./data/test_1.txt").expect("Failed to read test input file");
//         let result = part_process(&test_input).expect("Failed to process test input");
//         assert_eq!(result, 281);
//     }
// }

