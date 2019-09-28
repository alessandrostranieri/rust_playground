use rust_playground::sortbenchmark::{read_records_from_file, write_records_to_file};


fn main() {
    // READ RECORDS FROM FILE OPEN THE FILE
    let input_records = read_records_from_file("./data/minute.bin");
    // SORT
    let sorted = match input_records {
        Ok(mut records) => {
            records.sort_by_key(|record| record.key);
            Ok(records)
        },
        Err(msg) => Err(msg)
    };

    let result = match sorted {
        Ok(records) => write_records_to_file(&records, "./data/output.bin"),
        Err(msg) => Err(msg)
    };
    
    match result {
        Ok(_) => println!("Success! Terminating..."),
        Err(msg) => println!("Failure! Reason: {}", msg)
    };
}
