use ::indicatif::ProgressBar;
use chrono::DateTime;
use crossbeam_channel::unbounded;
use num_cpus;
use regex::Regex;
use std::fs;
use std::sync::mpsc;
use std::thread::{self, JoinHandle};
use DataStructures::RAWLogEntry;

pub fn parse_file_to_json(filename: String) -> Result<String, String> {
    println!("Reading file {}", &filename);
    let file_content = fs::read_to_string(filename).expect("Error reading file!");
    if file_content.is_empty() {
        return Err(String::from("File is empty!"));
    }
    let mut result_vector: Vec<RAWLogEntry> = Vec::new();
    println!("Converting into separate lines...");
    let input_lines = file_content.lines();
    let input_lines_converted = input_lines.collect::<Vec<&str>>();
    let input_size: u64 = input_lines_converted.len() as u64;
    let progress_bar = ProgressBar::new(input_size);
    let number_of_cpus = num_cpus::get();

    let (tx_main, rx_main) = unbounded();
    let (tx_child, rx_child) = unbounded();

    println!("Sending data into pipe...");
    let progress_bar_pipes = ProgressBar::new(input_size);
    for line in input_lines_converted {
        tx_main.send(String::from(line));
        progress_bar_pipes.inc(1);
    }
    progress_bar_pipes.finish_with_message("Sent all inputs into pipe");

    println!("Spawning threads...");
    for _thr in 0..number_of_cpus {
        let (tx_child_temp, _rx_child_temp) = (tx_child.clone(), rx_child.clone());
        let (_tx_main_temp, rx_main_temp) = (tx_main.clone(), rx_main.clone());
        thread::spawn(move || loop {
            let line = rx_main_temp.recv().unwrap();
            if line.contains("!kill_thread!") {
                break;
            }
            let my_regex = Regex::new(r"([0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}) (-) (-) (\[.*\]) (\W.*\W) ([0-9]{3}) ([0-9]*) (\W.*\W) (\W.*\W) (\W.*\W)").unwrap();
            let request_regex = Regex::new(r"\W(.*) (.*) (.*\W)").unwrap();
            let parsed_data = my_regex.captures(&line).unwrap();
            let source_ip = parsed_data.get(1).map_or("NULL", |m| m.as_str());
            let request_date = parsed_data.get(4).map_or("NULL", |m| m.as_str());
            let request_received = parsed_data.get(5).map_or("NULL", |m| m.as_str());
            let response_code_sent = parsed_data.get(6).map_or("NULL", |m| m.as_str());
            let bytes_sent = parsed_data.get(7).map_or("NULL", |m| m.as_str());
            let client_user_agent = parsed_data.get(9).map_or("NULL", |m| m.as_str());
            let parsed_request_data = request_regex.captures(&request_received).unwrap();
            let request_http_method_raw = parsed_request_data.get(1).map_or("NULL", |m| m.as_str());
            let request_http_method = request_http_method_raw.replace("\\\"", "");
            let request_endpoint = parsed_request_data.get(2).map_or("NULL", |m| m.as_str());
            let request_http_version_raw =
                parsed_request_data.get(3).map_or("NULL", |m| m.as_str());
            let request_http_version = request_http_version_raw.replace("\\\"", "");
            let timestamp = DateTime::parse_from_str(request_date, "[%d/%b/%Y:%H:%M:%S %z]");
            let timestamp_string = timestamp.unwrap().to_rfc2822();
            let log_entry: RAWLogEntry = RAWLogEntry {
                source_ip: (String::from(source_ip)),
                request_timestamp: (timestamp_string),
                request_http_method: (request_http_method),
                request_endpoint: (String::from(request_endpoint)),
                request_http_version: (request_http_version),
                response_code: (response_code_sent.parse::<i64>().unwrap()),
                response_bytes_count: (bytes_sent.parse::<i64>().unwrap()),
                http_client_user_agent: (String::from(client_user_agent)),
            };
            tx_child_temp.send(log_entry).unwrap();
        });
    }

    for received_data in rx_child {
        result_vector.push(received_data);
        progress_bar.inc(1);
    }

    for _thr in 0..number_of_cpus {
        tx_main.send(String::from("!kill_thread!"));
    }

    // for line in input_lines_converted {
    //     let my_regex = Regex::new(r"([0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}) (-) (-) (\[.*\]) (\W.*\W) ([0-9]{3}) ([0-9]*) (\W.*\W) (\W.*\W) (\W.*\W)").unwrap();
    //     let request_regex = Regex::new(r"\W(.*) (.*) (.*\W)").unwrap();
    //     let parsed_data = my_regex.captures(line).unwrap();
    //     let source_ip = parsed_data.get(1).map_or("NULL", |m| m.as_str());
    //     let request_date = parsed_data.get(4).map_or("NULL", |m| m.as_str());
    //     let request_received = parsed_data.get(5).map_or("NULL", |m| m.as_str());
    //     let response_code_sent = parsed_data.get(6).map_or("NULL", |m| m.as_str());
    //     let bytes_sent = parsed_data.get(7).map_or("NULL", |m| m.as_str());
    //     let client_user_agent = parsed_data.get(9).map_or("NULL", |m| m.as_str());
    //     let parsed_request_data = request_regex.captures(&request_received).unwrap();
    //     let request_http_method_raw = parsed_request_data.get(1).map_or("NULL", |m| m.as_str());
    //     let request_http_method = request_http_method_raw.replace("\\\"", "");
    //     let request_endpoint = parsed_request_data.get(2).map_or("NULL", |m| m.as_str());
    //     let request_http_version_raw = parsed_request_data.get(3).map_or("NULL", |m| m.as_str());
    //     let request_http_version = request_http_version_raw.replace("\\\"", "");
    //     let timestamp = DateTime::parse_from_str(request_date, "[%d/%b/%Y:%H:%M:%S %z]");
    //     let timestamp_string = timestamp.unwrap().to_rfc2822();
    //     let log_entry: RAWLogEntry = RAWLogEntry {
    //         source_ip: (String::from(source_ip)),
    //         request_timestamp: (timestamp_string),
    //         request_http_method: (request_http_method),
    //         request_endpoint: (String::from(request_endpoint)),
    //         request_http_version: (request_http_version),
    //         response_code: (response_code_sent.parse::<i64>().unwrap()),
    //         response_bytes_count: (bytes_sent.parse::<i64>().unwrap()),
    //         http_client_user_agent: (String::from(client_user_agent)),
    //     };

    //     result_vector.push(log_entry);
    //     progress_bar.inc(1);
    //     //println!("{}", serialized_json_string.unwrap());
    // }
    let serialized_json_string = serde_json::to_string(&result_vector);
    progress_bar.finish_with_message("Finished processing file\n");
    return Ok(serialized_json_string.unwrap());
}
