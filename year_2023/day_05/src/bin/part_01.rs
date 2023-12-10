use std::fs;
use std::ops::Range;

use nom::{
    bytes::complete::take_until,
    character::complete::{self, line_ending, space1},
    multi::{many1, separated_list1},
    sequence::tuple,
    IResult, Parser,
};
use nom_supreme::{tag::complete::tag, ParserExt};
use std::error::Error;
use std::fmt;

// Define your own error type, if needed
#[derive(Debug)]
struct ProcessError {
    message: String,
}

impl fmt::Display for ProcessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Process error: {}", self.message)
    }
}

impl Error for ProcessError {}

struct SeedMap {
    mappings: Vec<(Range<u64>, Range<u64>)>,
}

impl SeedMap {
    fn translate(&self, source: u64) -> u64 {
        let valid_mapping = self
            .mappings
            .iter()
            .find(|(source_range, _)| source_range.contains(&source));

        let Some((source_range, destination_range)) =
            valid_mapping
        else {
            return source;
        };

        let offset = source - source_range.start;

        destination_range.start + offset
    }
}

fn main() {
    println!("Day 5 part 1");
    let input = fs::read_to_string("./data/input.txt").expect("Failed to read input file");
    let result = process(&input);
    println!("Result: {:?}", result);
}

fn line(input: &str) -> IResult<&str, (Range<u64>, Range<u64>)> {
    let (input, (destination, source, num)) = tuple((
        complete::u64,
        complete::u64.preceded_by(tag(" ")),
        complete::u64.preceded_by(tag(" ")),
    ))(input)?;

    Ok((
        input,
        (source..(source + num), destination..(destination + num)),
    ))
}
fn seed_map(input: &str) -> IResult<&str, SeedMap> {
    take_until("map:")
        .precedes(tag("map:"))
        .precedes(many1(line_ending.precedes(line)).map(|mappings| SeedMap { mappings }))
        .parse(input)
}

fn parse_seedmaps(input: &str) -> IResult<&str, (Vec<u64>, Vec<SeedMap>)> {
    let (input, seeds) = tag("seeds: ")
        .precedes(separated_list1(space1, complete::u64))
        .parse(input)?;
    let (input, maps) = many1(seed_map)(input)?;

    Ok((input, (seeds, maps)))
}

pub fn process(input: &str) -> Result<String, Box<dyn Error>> {
    let (_, (seeds, maps)) = parse_seedmaps(input).map_err(|e| {
        Box::new(ProcessError {
            message: e.to_string(),
        }) as Box<dyn Error>
    })?;

    let locations = seeds
        .iter()
        .map(|seed| maps.iter().fold(*seed, |seed, map| map.translate(seed)))
        .collect::<Vec<u64>>();

    locations
        .iter()
        .min()
        .map(|location| location.to_string())
        .ok_or_else(|| {
            Box::new(ProcessError {
                message: "No minimum location value found".to_string(),
            }) as Box<dyn Error>
        })
}

// pub fn process(input: &str) -> Result<String, Box<dyn Error>> {
//     let (_, (seeds, maps)) = parse_seedmaps(input).map_err(|e| ProcessError {
//         message: e.to_string(),
//     })?;
//
//     let locations = seeds
//         .iter()
//         .map(|seed| maps.iter().fold(*seed, |seed, map| map.translate(seed)))
//         .collect::<Vec<u64>>();
//
//     locations
//         .iter()
//         .min()
//         .map(|location| location.to_string())
//         .ok_or_else(|| {
//             Box::new(ProcessError {
//                 message: "No minimum location value found".to_string(),
//             })
//         })
// }

// pub fn process(input: &str) -> miette::Result<String, _> {
//     let (_, (seeds, maps)) = parse_seedmaps(input).expect("a valid parse");
//
//     let locations = seeds
//         .iter()
//         .map(|seed| maps.iter().fold(*seed, |seed, map| map.translate(seed)))
//         .collect::<Vec<u64>>();
//
//     Ok(locations
//         .iter()
//         .min()
//         .expect("should have a minimum location value")
//         .to_string())
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), Box<dyn Error>> {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!("35", process(input)?);
        Ok(())
    }
}
