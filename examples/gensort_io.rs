use rust_playground::sortbenchmark::{GensortRecord, read_records};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use itertools::chain;

fn main() {
    // OPEN THE FILE
    let input_file = File::open("./data/minute.bin");
    let r = match input_file {
        Ok(file) => read_records(file),
        Err(msg) => Err(format!("Error opening file. Reason: {}", msg))
    };
    // SORT
    let mut records: Vec<GensortRecord> = r.unwrap();

    records.sort_by_key(|r| r.key);

    let output_file = File::create("./data/output.bin");
    match output_file {
        Ok(writer) => {
            let mut buffered_writer = BufWriter::new(writer);
            let mut temp_buffer: [u8; 100] = [0u8; 100];
            let res: Result<Vec<_>, _> = records
                .iter()
                .map(|record| {
                    temp_buffer[..10].copy_from_slice(&record.key[..]);
                    temp_buffer[10..].copy_from_slice(&record.value[..]);
                    buffered_writer.write_all(&temp_buffer)
                })
                .collect();
        }
        Err(msg) => println!("Error during writing"),
    }
}
