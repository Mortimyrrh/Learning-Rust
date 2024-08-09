use std::fs;
use std::env::args;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>{
    // S1 P2 - In for the long haul
    
    // Get file name from args.
    let file_name = args().nth(1).ok_or("Expected filename")?;

    // Get file content as string.
    let file_contents = fs::read_to_string(&file_name)?; // Should be using `BufRead`
    

    // Valid command chars
    let valid = ['>', '<', '+', '-', '.', ',', '[', ']'];

    // Get only the valid chars
    let parsed: Vec<char> = file_contents.to_string().chars().filter(|c| valid.contains(c)).collect();
    
    // Format for printing
    let output: String = parsed.iter().collect();

    println!("{output}");
    
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
