use std::env::args;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    // S1 P2 - In for the long haul

    // Get file name from args.
    let file_name = args().nth(1).ok_or("Expected filename")?;

    // Get file content as string.
    let file_contents = fs::read_to_string(&file_name)?; // Should be using `BufRead`

    // // Remove whitespace
    // let parsed: Vec<char> = file_contents.to_string().chars().filter(|c| !c.is_whitespace()).collect();

    // // Remove bookends
    // let file_bookend: Vec<char>  = "---8<---".chars().collect();
    // let start = if parsed.starts_with(&file_bookend) {file_bookend.len()} else {0};
    // let end = if parsed.ends_with(&file_bookend) {file_bookend.len()} else {0};
    // let body: Vec<char> = parsed[start..parsed.len()-end].to_vec();

    // // Remove invalid chars
    let valid_chars = ['>', '<', '+', '-', '.', ',', '[', ']'];
    // let valid: Vec<char> = body.clone().into_iter().filter(|c| valid_chars.contains(c)).collect(); // Feel like I should be using borrow/mut to avoid the .clone().into_iter()

    let parsed: Vec<char> = file_contents
        .chars()
        .filter(|c| valid_chars.contains(c))
        .collect();

    println!("{}", parsed.iter().collect::<String>());

    Ok(())
}

// naive approach
// for c in file_contents.to_string().chars()
// {
//     match c {
//         '>'| '<'| '+'| '-'| '.'| ','|'[' | ']' => parsed.push(c),
//         _ => (),
//     };
// }
// dbg!(parsed);
