use tokio::prelude::*;
use tokio::codec::{FramedRead, LinesCodec};
use tokio::io::{self, AsyncWriteExt};
use tokio::fs::File;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut reader = FramedRead::new(stdin, LinesCodec::new());
    let line = reader.next().await.transpose()?.unwrap();
    println!("You typed: {}", line);
    let mut f = File::create("out.txt").await?;
    let line = format!("LOG: read a line: {}\n", line);
    f.write_all(line.as_bytes()).await?;
    Ok(())
}
