use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn get_arg_or_die(num: usize) -> Option<String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= num {
        return None;
    }
    Some(args[num].clone())
}

pub fn read_file_as_lines(file_name: &String) -> Vec<String> {
    let file = File::open(file_name).expect("File could not be open");
    let buffer = BufReader::new(file);

    buffer.lines().map(|l| l.expect("Could not parse line")).collect()
}

