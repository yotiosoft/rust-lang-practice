#![feature(buf_read_has_data_left)]
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let stdin = io::stdin();

    let mut reader = BufReader::new(stdin);
    let mut buffer = Vec::new();

    loop {
        buffer.clear();
        let bytes_read = reader.read_until(b'\n', &mut buffer)?;
        
        if buffer[0] == b'\n' || buffer[0] == b'\r' {
            println!("Empty line");
        }
        else {
            println!("Non-empty line");
        }
        
        println!("{} {}", buffer.len(), buffer[0]);
        if reader.has_data_left()? == false {
            println!("No more data left");
        }
        else {
            println!("More data left");
        }
        if bytes_read == 0 {
            break;
        }
    }

    Ok(())
}
