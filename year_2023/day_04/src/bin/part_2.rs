use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// Main function that reads scratchcard data from a file and prints the total number of scratchcards.
fn main() -> io::Result<()> {
    let path = "./data/input.txt";
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // New code to print the first three lines
    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        if index < 3 {
            println!("{}", line);
        } else {
            break;
        }
    }

    // Process the cards
    let file = File::open(path)?; // Re-open the file as the previous reader is consumed
    let reader = BufReader::new(file);
    let total_cards = process_cards(reader.lines());
    println!("Total number of scratchcards: {}", total_cards);
    Ok(())
}

/// Processes scratchcard data from an iterator over lines.
///
/// # Arguments
///
/// * `lines` - An iterator over `Result<String, io::Error>`, typically lines from a file.
///
/// # Returns
///
/// * The total number of scratchcards, including originals and copies.
fn process_cards<I>(lines: I) -> usize
where
    I: Iterator<Item = io::Result<String>>,
{
    // Parse each line to create a list of cards
    let mut cards = Vec::new();

    for line in lines {
        if let Ok(card) = line {
            let card_data = card.split(':').nth(1).unwrap_or("").trim();
            let parts: Vec<_> = card_data.split('|').collect();
            let left: Vec<_> = parts[0]
                .trim()
                .split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect();
            let right: Vec<_> = parts[1]
                .trim()
                .split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect();
            cards.push((left, right));
        }
    }

    // Process the cards to calculate total number
    let mut total_cards = 0;
    let mut to_process = vec![1; cards.len()];

    for i in 0..cards.len() {
        let (ref left, ref right) = cards[i];
        let matches = left.iter().filter(|&&n| right.contains(&n)).count();
        total_cards += to_process[i];

        for j in 1..=matches {
            if i + j < cards.len() {
                to_process[i + j] += to_process[i];
            }
        }
    }

    total_cards
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the `process_cards` function with a predefined set of scratchcard data.
    #[test]
    fn test_scratchcards() {
        let input_data = " Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let lines = input_data.split('\n').map(String::from).map(Ok);

        assert_eq!(process_cards(lines), 30);
    }
}
