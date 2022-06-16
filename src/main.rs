use clap::Parser;
use std::fs;
use FileHandler::parse_file_to_json;
/// Parser for the data warehouse
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Filename of the input file
    #[clap(short, long, value_parser)]
    input_file: String,

    /// Filename of the output file
    #[clap(short, long, value_parser, default_value = "output.log.json")]
    output_file: String,
}
fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();
    let result = parse_file_to_json(String::from(args.input_file));
    let _output_file = fs::write(args.output_file, result.unwrap());
    return Ok(());
}
