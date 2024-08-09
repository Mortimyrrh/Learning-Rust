
use std::fs;
use std::env::args;
use std::error::Error;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Session 1 - Problem 1: Adding up numbers");
    
    // Get file name from args.
    let file_name = args().nth(1).ok_or("Expected filename")?;
    println!("File Name: {file_name}");

    // Get file content as string.
    let file_contents = fs::read_to_string(&file_name)?;

    let mut file_sum: i128 = 0;

    // format into lines
    for line in file_contents.lines()
    {
        // add if not the file bookend
        file_sum += match line.trim() {
            "---8<---" => 0, // why can't I use this with // let file_bookend: &str = "---8<---"; // it just overrides
            l => i128::from_str(l)?
        };
    }
    print!("Sum of numbers in {file_name} is: {file_sum}\n");

    Ok(())
}
