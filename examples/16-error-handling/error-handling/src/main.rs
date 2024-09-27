use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn main() {
    let file = File::open("non_existent_file.txt");
    let file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::PermissionDenied => {
                println!("Permission denied: {}", error);
                return;
            }
            _ => {
                panic!("Error opening file: {}", error);
            }
        },
    };

    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => {
                panic!("Error reading line: {}", error);
            }
        }
    }

    let output_file = File::create("output.txt");
    match output_file {
        Ok(mut file) => match file.write_all(b"Hello, World!") {
            Ok(_) => println!("Write operation successful"),
            Err(error) => panic!("Error writing to file: {}", error),
        },
        Err(error) => panic!("Error creating file: {}", error),
    }
}
//
