use rust_playground::sortbenchmark::read_records;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn main() {
    // OPEN THE FILE
    let f = File::open("./data/minute.bin");
    let r = match f {
        Ok(file) => read_records(file),
        Err(msg) => Err(msg)
    };
    match r {
        Ok(records) => println!("Number or records: {}", records.len()),
        Err(msg) => println!("Error: {}", msg)
    }
}
