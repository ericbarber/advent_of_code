use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    println!("Day 1 part 2");
    let input = read_lines("./data/input.txt").expect("Failed to read input file");
    let result = part_process(&input).expect("Failed to process input");
    println!("Result: {}", result);
}

fn part_process(input: &[String]) -> Result<u32, &'static str> {
    let mut sum = 0;

    for line in input {
        let digits: Vec<u32> = extract_digits(&line)
            .iter()
            .map(|&(c, _)| c.to_digit(10).unwrap())
            .collect();

        let line_result = match digits.len() {
            0 => return Err("No digits found in a line"),
            1 => digits[0] * 11,
            _ => digits[0] * 10 + digits[digits.len() - 1],
        };
        // println!("{}\n", line_result);

        sum += line_result;
    }

    Ok(sum)
}

fn extract_digits(input: &str) -> Vec<(char, usize)> {
    let words_to_digits = [
        ("zero", '0', 4), ("one", '1', 3), ("two", '2', 3), ("three", '3', 5),
        ("four", '4', 4), ("five", '5', 4), ("six", '6', 3), ("seven", '7', 5),
        ("eight", '8', 5), ("nine", '9', 4),
    ];

    let mut result = Vec::new();
    let mut chars = input.chars().enumerate().peekable();

    while let Some((index, c)) = chars.next() {
        if c.is_digit(10) {
            result.push((c, index));
        } else if c.is_alphabetic() {

            for &(word, digit, length) in &words_to_digits {
                let next_chars: String = chars.clone().take(length - 1).map(|(_, c)| c).collect();
                let word_candidate = format!("{}{}", c, next_chars);
                // println!("{} \t {} \t {} \t {}", input, index, word_candidate, word);

                if word_candidate == word {
                    result.push((digit, index));
                    // println!("******\n{}\t{}\t{}\n******\n", word, index, digit);
                    break;
                }
            }
        }
    }
    // println!("{:?}", result);

    result
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
            "two1nine".to_string(),
            "eightwothree".to_string(),
            "abcone2threexyz".to_string(),
            "xtwone3four".to_string(),
            "4nineeightseven2".to_string(),
            "zoneight234".to_string(),
            "7pqrstsixteen".to_string(),
            // Testing the overlap of two words
            "threetwone".to_string(),
        ];

        let expected_results = vec![29, 83, 13, 24, 42, 14, 76, 31];
        let mut actual_results = Vec::new();

        for line in test_input {
            let digits: Vec<u32> = extract_digits(&line)
                .iter()
                .map(|&(c, _)| c.to_digit(10).unwrap())
                .collect();

            let line_result = match digits.len() {
                0 => panic!("No digits found in a line"),
                1 => digits[0] * 11,
                _ => digits[0] * 10 + digits[digits.len() - 1],
            };

            actual_results.push(line_result);
        }

        assert_eq!(actual_results, expected_results);
    }
}
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_part_one() {
//         let test_input = read_lines("./data/test_2.txt").expect("Failed to read test input file");
//         let result = part_process(&test_input).expect("Failed to process test input");
//         assert_eq!(result, 281);
//     }
// }
