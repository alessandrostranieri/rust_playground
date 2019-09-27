use std::fs::File;
use std::io::{Read, BufReader, BufRead};
use std::io::Error;
use std::cmp::Ordering;
use std::mem;

pub struct GensortRecord {
    pub key: [u8; 10],
    pub value: [u8; 90]
}

impl GensortRecord {
    fn new(b: &[u8]) -> GensortRecord {
        let mut r = GensortRecord {
            key: [0u8; 10],
            value: [0u8; 90]
        };
        r.key.copy_from_slice(&b[..10]);
        r.value.copy_from_slice(&b[10..100]);
        r
    }
}

pub fn read_records(file: File) -> Result<Vec<GensortRecord>, Error> {
    let file_data_len = match file.metadata() {
        Ok(metadata) => Ok(metadata.len()),
        Err(msg) => Err(msg)
    };

    const record_size: usize = mem::size_of::<GensortRecord>();
    let num_records = match file_data_len {
        Ok(num_bytes) => 
            match (num_bytes as usize).checked_rem(record_size) {
                Some(0) => Ok((num_bytes as usize) / record_size),
                _ => Err(format!("Invalid number of bytes: {}", num_bytes))
            },
        Err(msg) => Err("Yikes".to_string())
    };

    match num_records {
        Ok(num_records) => println!("Number of records: {}", num_records),
        Err(msg) => println!("Something went wrong {}", msg) 
    };

    let mut reader: BufReader<File> = BufReader::new(file);
    let mut records: Vec<GensortRecord> = Vec::new();

    let mut base = [0u8; record_size];
    for i in 0..10 {
        // READ DATA
        reader.read_exact(&mut base);
        let s = GensortRecord::new(&base);
        records.push(s);      
    }
    
    Ok(records)
}

#[cfg(test)]
mod tests {

    use std::mem;
    use crate::sortbenchmark::GensortRecord;

    #[test]
    fn test_gensort_size() {
        assert_eq!(100usize, mem::size_of::<GensortRecord>());
    }
}