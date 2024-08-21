use core::fmt;
use std::collections::HashMap;
use std::env::args;
use std::error::Error;
use std::fs;

#[derive(Debug)]
enum LineData {
    NameAndNumber(String, u32),
    NameOnly(String),
}

impl TryFrom<&str> for LineData {
    type Error = Box<dyn Error>;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        match line.split_once(':') {
            Some(s) => Ok(LineData::NameAndNumber(s.0.to_owned(), s.1.parse()?)),
            None => Ok(LineData::NameOnly(line.to_string())),
        }
    }
}

fn parse_file(file_name: &str) -> Result<Vec<LineData>, Box<dyn Error>> {
    // file name is not a string
    let file_contents = fs::read_to_string(file_name)?; // reads as chars and not u8 (this increases memory usage 4x)

    let mut data: Vec<LineData> = Vec::new();

    for line in file_contents.lines() {
        data.push(line.try_into()?);
    }

    Ok(data)
}

fn main() -> Result<(), Box<dyn Error>> {
    // S2 P1 - Practice for structs and enumerations

    // Get file name from args.
    let file_name = args().nth(1).ok_or("Expected filename")?;

    let file_data = parse_file(&file_name)?;

    let mut score_cards: HashMap<String, ScoreCard> = HashMap::new();

    for l in file_data {
        match l {
            LineData::NameOnly(name) => score_cards.entry(name).or_default().missed_test(),
            LineData::NameAndNumber(name, number) => {
                score_cards.entry(name).or_default().add_score(number)
            }
        }
    }

    for (name, card) in score_cards {
        println!("{name} took {card}");
    }

    Ok(())
}

#[derive(Default)]
struct ScoreCard {
    running_total: u32,
    tests_taken: u32,
    missed_tests: u32,
}

impl ScoreCard {
    fn add_score(&mut self, new_score: u32) {
        self.running_total += new_score;
        self.tests_taken += 1;
    }

    fn missed_test(&mut self) {
        self.missed_tests += 1;
    }

    fn running_total(&self) -> u32 {
        self.running_total
    }

    fn tests_taken(&self) -> u32 {
        self.tests_taken
    }

    fn missed_tests(&self) -> u32 {
        self.missed_tests
    }
}

impl fmt::Display for ScoreCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}, with a total score of {}. They missed {} {}.",
            self.tests_taken(),
            if self.tests_taken() == 1 {
                "test"
            } else {
                "tests"
            },
            self.running_total(),
            self.missed_tests(),
            if self.missed_tests() == 1 {
                "test"
            } else {
                "tests"
            }
        )
    }
}
