use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("Day 2 part 2");
    let input = read_lines("./data/input.txt").expect("Failed to read input file");
    let result = part_process(&input).expect("Failed to process input");
    println!("Result: {}", result);
}

fn part_process(input: &[String]) -> Result<u32, &'static str> {
    let mut sum = 0;

    for (_index, line) in input.iter().enumerate() {
        // sum game power as returned
        let power = process_game_line(line);
        sum += power;

        if let Some((game_id, _)) = line.split_once(':') {
            println!("Game ID: {}, Power: {}, Sum: {}", game_id.trim(), power, sum);
        } else {
            println!("No colon found in line: {}", line);
        }
    }
    Ok(sum.try_into().unwrap())
}

fn process_game_line(line: &str) -> usize {
    let mut max_cubes = [("red", 0), ("green", 0), ("blue", 0)].iter().cloned().collect::<std::collections::HashMap<_, _>>();

    // Split the line at the colon
    let parts: Vec<&str> = line.splitn(2, ':').collect();

    // Check if we have two parts (game info and plays)
    if parts.len() == 2 {
        // The second part contains the plays, split it by semicolon
        let plays = parts[1].split(';');

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
                let entry = max_cubes.entry(color).or_insert(0);
                if count > *entry {
                    *entry = count;
                    println!("\tUpdating {} to {}", color, count);
                }
            }
        }
    }
    // multiply the max value for the cubes
    max_cubes.values().map(|&val| val as usize).product()
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
            // "Game 6: 12 red, 14 blue, 13 green; 14 blue, 12 red, 13 green".to_string(),
        ];

        let expected_result = 48 + 12 + 1560 + 630 + 36; // Sum of game numbers that did not exceed the limit
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

