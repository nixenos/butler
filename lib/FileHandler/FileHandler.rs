use std::fmt::*;
use std::fs::*;

fn parse_file_to_json(filename: String) -> String {
    let file_content = fs::read_to_string(filename)?.parse()?;
    
}