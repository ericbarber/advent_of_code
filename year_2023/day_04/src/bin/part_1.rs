use std::fs;

fn main() {
    println!("Day 4 part 1");
    let input = fs::read_to_string("./data/input.txt").expect("Failed to read input file");
    let sum = sum_card_value(&input);
    println!("Sum of part numbers: {}", sum);
}

fn sum_card_value(input: &str) -> u32 {
    let lines: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            let mut parts = line.split(" | ");
            let winning_numbers: Vec<u32> = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();
            let your_numbers: Vec<u32> = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();
            (winning_numbers, your_numbers)
        })
        .collect();

    let mut sum = 0;
    for (winning_numbers, your_numbers) in lines {
        let mut card_wins = 0;
        for val in your_numbers {
            if winning_numbers.contains(&val) {
                card_wins += 1;
            }
        }
        sum += card_wins;
    }
    sum
}

#[cfg(test)]
mod tests {

    #[test]
    fn sum_card_value() {
        let test_input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11    
";

        let expected_result = 13; // Sum of part numbers in the example
        let actual_result = sum_card_value(&test_input);

        assert_eq!(actual_result, expected_result);
    }
}
