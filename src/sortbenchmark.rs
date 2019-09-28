use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::mem;
use std::path::Path;
use file_diff::{diff};

pub struct GensortRecord {
    pub key: [u8; 10],
    pub value: [u8; 90],
}

impl GensortRecord {
    fn new(b: &[u8]) -> GensortRecord {
        let mut r = GensortRecord {
            key: [0u8; 10],
            value: [0u8; 90],
        };
        r.key.copy_from_slice(&b[..10]);
        r.value.copy_from_slice(&b[10..100]);
        r
    }
}

const RECORD_SIZE: usize = mem::size_of::<GensortRecord>();

pub fn read_records_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<GensortRecord>, String> {
    let input_file = File::open(path);
    match input_file {
        Ok(file) => read_records(file),
        Err(msg) => Err(format!("Error opening file. Reason: {}", msg)),
    }
}

pub fn calculate_number_of_records(file: &File) -> Result<usize, String> {
    let file_data_len = match file.metadata() {
        Ok(metadata) => Ok(metadata.len()),
        Err(msg) => Err(format!("Error obtaining metadata. Reason: {}", msg)),
    };

    let num_records = match file_data_len {
        Ok(num_bytes) => match (num_bytes as usize).checked_rem(RECORD_SIZE) {
            Some(0) => Ok((num_bytes as usize) / RECORD_SIZE),
            _ => Err(format!("Invalid number of bytes: {}", num_bytes)),
        },
        Err(msg) => Err(msg),
    };

    num_records
}

pub fn read_records(file: File) -> Result<Vec<GensortRecord>, String> {
    let num_records = calculate_number_of_records(&file);

    let records = match num_records {
        Ok(num_records) => {
            let mut reader: BufReader<File> = BufReader::new(file);
            let mut records: Vec<GensortRecord> = Vec::new();
            records.reserve(num_records);
            let mut base = [0u8; RECORD_SIZE];
            for _ in 0..num_records {
                let _ = reader.read_exact(&mut base);
                let s = GensortRecord::new(&base);
                records.push(s);
            }
            Ok(records)
        }
        Err(msg) => Err(msg),
    };

    records
}

pub fn write_records_to_file<P: AsRef<Path>>(
    records: &Vec<GensortRecord>,
    path: P,
) -> Result<(), String> {
    let output_file = File::create(path);
    match output_file {
        Ok(writer) => {
            let mut buffered_writer = BufWriter::new(writer);
            let mut temp_buffer: [u8; 100] = [0u8; RECORD_SIZE];
            let writing: Result<Vec<_>, _> = records
                .iter()
                .map(|record| {
                    temp_buffer[..10].copy_from_slice(&record.key[..]);
                    temp_buffer[10..].copy_from_slice(&record.value[..]);
                    buffered_writer.write_all(&temp_buffer)
                })
                .collect();
            match writing {
                Ok(_) => Ok(()),
                Err(msg) => Err(format!("Error writing records. Reason: {}", msg))
            }
        }
        Err(msg) => Err(format!("Error creating output file. Reason: {}", msg)),
    }
}

#[cfg(test)]
mod tests {

    use crate::sortbenchmark::{RECORD_SIZE, read_records_from_file, write_records_to_file};

    #[test]
    fn test_gensort_size() {
        assert_eq!(100usize, RECORD_SIZE);
    }

    #[test]
    fn test_full_process() {
        // READ RECORDS
        let mut records = read_records_from_file("./data/gensort/test_input.bin").unwrap();
        records.sort_by_key(|record| record.key);
        assert_eq!(records.len(), 1000);
        write_records_to_file(&records, "./data/gensort/temp_output.bin").unwrap();
        // COMPARE FILES
        assert!(file_diff::diff("./data/gensort/test_output.bin", "./data/gensort/temp_output.bin"));
    }
}
