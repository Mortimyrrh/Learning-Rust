use core::fmt;
use std::env::args;
use std::error::Error;
use std::ffi::OsStr;
use std::fmt::write;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    // S2 P2 - Another step toward an interpreter

    let file_name = args().nth(1).ok_or("Expected filename")?;

    let instructions = parse_file(file_name.into())?;

    for instruction in instructions {
        println!("{}", instruction);
    }

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

enum RawInstruction {
    MoveRight,
    MoveLeft,
    IncrementData,
    DecrementData,
    Output,
    Input,
    BeginLoop,
    EndLoop,
}

impl RawInstruction {
    fn from_char(c: char) -> Option<RawInstruction> {
        match c {
            '>' => Some(RawInstruction::MoveRight),
            '<' => Some(RawInstruction::MoveLeft),
            '+' => Some(RawInstruction::IncrementData),
            '-' => Some(RawInstruction::DecrementData),
            '.' => Some(RawInstruction::Output),
            ',' => Some(RawInstruction::Input),
            '[' => Some(RawInstruction::BeginLoop),
            ']' => Some(RawInstruction::EndLoop),
            _ => None,
        }
    }
}

impl fmt::Display for RawInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RawInstruction::MoveRight => write!(f, "Move Right"),
            RawInstruction::MoveLeft => write!(f, "Move Left"),
            RawInstruction::IncrementData => write!(f, "Increment Data"),
            RawInstruction::DecrementData => write!(f, "Decrement Data"),
            RawInstruction::Output => write!(f, "Output"),
            RawInstruction::Input => write!(f, "Input"),
            RawInstruction::BeginLoop => write!(f, "Begin Loop"),
            RawInstruction::EndLoop => write!(f, "End Loop"),
        }
    }
}

struct InputInstruction {
    raw_instruction: RawInstruction,
    line_number: usize,
    char_column: usize,
    file_path: PathBuf,
}

// impl InputInstruction {
//     fn get_file_name(self) -> String {
//         match self.file_path.file_name() {
//             Some(name) => name.to_owned().to_string_lossy(), // I can't work out how to get the PathBuff back to a String
//             None => "".to_owned()
//         }
//     }
// }

fn parse_file(path: PathBuf) -> Result<Vec<InputInstruction>, Box<dyn Error>> {
    let file_contents = fs::read_to_string(&path)?;

    let mut instructions: Vec<InputInstruction> = Vec::new();

    let mut line_number = 1;
    let mut char_column = 1;

    for line in file_contents.lines() {
        for character in line.chars() {
            match RawInstruction::from_char(character) {
                Some(raw_instruction) => instructions.push(InputInstruction {
                    raw_instruction: raw_instruction,
                    line_number: line_number,
                    char_column: char_column,
                    file_path: path.clone(),
                }),
                None => (), // ignore invalid characters for now,
            }
            char_column += 1;
        }
        line_number += 1;
    }

    Ok(instructions)
}

impl fmt::Display for InputInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{:?}:{}:{}] {}",
            self.file_path.file_name(),
            self.line_number,
            self.char_column,
            self.raw_instruction
        )
    }
}
