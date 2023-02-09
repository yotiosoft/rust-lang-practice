#![feature(buf_read_has_data_left)]
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let stdin = io::stdin();

    let mut reader = BufReader::new(stdin);
    let mut buffer = Vec::new();

    loop {
        buffer.clear();
        let bytes_read = reader.read_until(b'\n', &mut buffer)?;
        println!("{}", buffer.len());
        if reader.has_data_left()? == false {
            println!("No more data left");
        }
        if bytes_read == 0 {
            break;
        }
    }

    Ok(())
}
