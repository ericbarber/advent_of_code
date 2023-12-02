use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    println!("Day 1 part 1");
    let input = read_lines("./data/input.txt").expect("Failed to read input file");
    let result = part_one(&input).expect("Failed to process input");
    println!("Result: {}", result);
}

fn part_one(input: &[String]) -> Result<u32, &'static str> {
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

        sum += line_result;
    }

    Ok(sum)
}
fn extract_digits(input: &str) -> Vec<(char, usize)> {
    input
        .char_indices()
        .filter(|&(_, c)| c.is_digit(10))
        .map(|(index, c)| (c, index)) // Swap the elements in the tuple
        .collect()
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
    fn test_part_one() {
        let test_input = read_lines("./data/test_1.txt").expect("Failed to read test input file");
        let result = part_one(&test_input).expect("Failed to process test input");
        assert_eq!(result, 142);
    }
}
