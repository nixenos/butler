use std::fmt::*;
use std::fs::*;

fn open_file(filename: String) -> File {
    let opened_file = File::open(filename);
    println!("Opening file: {}", filename);
    return opened_file.unwrap();
}

fn parse_file_to_json(data_file: File) -> String {

}