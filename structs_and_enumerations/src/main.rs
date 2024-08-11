use std::fs;
use std::env::args;
use std::error::Error;

#[derive(Debug)]
enum LineData {
    NameAndNumber{name: String, number: u32},
    NameOnly{name: String}
}

impl TryFrom<&str> for LineData {
    type Error = Box<dyn Error>;

    fn try_from(line: &str) -> Result<Self, Self::Error> {

        if !line.contains(':')
        {
            return Ok(LineData::NameOnly{name: line.to_string()});
        } else {
            let parts: Vec<&str> = line.splitn(2, ":").collect();

            let na: &str = parts[0];
            let nu: u32 = parts[1].parse::<u32>()?;
            return Ok(LineData::NameAndNumber{name: na.to_string(), number: nu})
        }
        return Ok(LineData::NameOnly{name: "mortimer".to_string()});

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
    
    for l in file_data {
        println!("{:?}", l);
    }
    
    Ok(())
}

