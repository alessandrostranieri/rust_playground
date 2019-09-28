use std::cmp::Ordering;
use std::fs::File;
use std::io::Error;
use std::io::{BufRead, BufReader, Read};
use std::mem;

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
                // READ DATA
                reader.read_exact(&mut base);
                let s = GensortRecord::new(&base);
                records.push(s);
            }
            Ok(records)
        }
        Err(msg) => Err(msg),
    };

    records
}

#[cfg(test)]
mod tests {

    use crate::sortbenchmark::GensortRecord;
    use std::mem;

    #[test]
    fn test_gensort_size() {
        assert_eq!(100usize, mem::size_of::<GensortRecord>());
    }
}
