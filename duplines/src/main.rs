use tokio::io::{self, BufReader};

#[tokio::main]
async fn main() {
    let mut reader = BufReader::new(io::stdin());
    let mut lines = Vec::new();

    loop {
        let mut line = String::new();
        let n = reader.buffer().len();

        if n == 0 {
            break;
        }

        lines.push(line);
    }

    println!("There's: {}", lines.len());
    println!("{}", lines.join("\n"));
}
