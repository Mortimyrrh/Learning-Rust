use std::fs;
use std::env::args;
use std::error::Error;
use std::collections::HashMap;

#[derive(Debug)]
enum LineData {
    NameAndNumber(String, u32),
    NameOnly(String)
}

impl TryFrom<&str> for LineData {
    type Error = Box<dyn Error>;

    fn try_from(line: &str) -> Result<Self, Self::Error> {

        if !line.contains(':')
        {
            return Ok(LineData::NameOnly(line.to_string()));
        } else {
            let parts: Vec<&str> = line.splitn(2, ":").collect();

            let name: &str = parts[0];
            let number: u32 = parts[1].parse::<u32>()?;
            return Ok(LineData::NameAndNumber(name.to_string(), number))
        }
    }
}

fn parse_file(file_name: &str) -> Result<Vec<LineData>, Box<dyn Error>>
{
    let file_contents = fs::read_to_string(&file_name)?;

    let mut data: Vec<LineData> = Vec::new();

    for line in file_contents.lines() {
        data.push(<LineData>::try_from(line)?);
    }

    Ok(data)
}

fn main() -> Result<(), Box<dyn Error>>{
    // S2 P1 - Practice for structs and enumerations
    
    // Get file name from args.
    let file_name = args().nth(1).ok_or("Expected filename")?;

    let file_data = parse_file(&file_name)?;

    let mut score_cards:HashMap<String, ScoreCard>;
    
    for l in file_data {

        match score_cards {
            LineData::NameOnly(name) => score_cards.entry(name).or_insert(ScoreCard::default())
        }

        println!("{:?}", l);
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

    fn get_total(&self) -> u32 {
        self.running_total
    }

    fn get_tests_taken(&self) -> u32 {
        self.tests_taken
    }

    fn get_missed_tests(&self) -> u32 {
        self.missed_tests
    }
}
