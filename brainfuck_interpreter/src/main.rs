use core::fmt;
use std::env::args;
use std::error::Error;
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
            RawInstruction::MoveRight => write!(f, "MoveRight"),
            RawInstruction::MoveLeft => write!(f, "MoveLeft"),
            RawInstruction::IncrementData => write!(f, "IncrementData"),
            RawInstruction::DecrementData => write!(f, "DecrementData"),
            RawInstruction::Output => write!(f, "Output"),
            RawInstruction::Input => write!(f, "Input"),
            RawInstruction::BeginLoop => write!(f, "BeginLoop"),
            RawInstruction::EndLoop => write!(f, "EndLoop"),
        }
    }
}

struct InputInstruction {
    raw_instruction: RawInstruction,
    line_number: usize,
    char_column: usize,
}

fn parse_file(path: PathBuf) -> Result<Vec<InputInstruction>, Box<dyn Error>> {
    let file_contents = fs::read_to_string(&path)?;

    let mut instructions: Vec<InputInstruction> = Vec::new();

    let mut line_number = 0;
    let mut char_column = 0;

    for line in file_contents.lines() {
        for character in line.chars() {
            match RawInstruction::from_char(character) {
                Some(raw_instruction) => instructions.push(InputInstruction {
                    raw_instruction: raw_instruction,
                    line_number: line_number,
                    char_column: char_column,
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
            "{} at line:{}, col:{}",
            self.raw_instruction, self.line_number, self.char_column
        )
    }
}


