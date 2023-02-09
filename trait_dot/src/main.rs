/// トレイトプログラム
/// 11.5章（第2版）pp.259-260

use std::io::Write;

fn say_hello(out: &mut dyn Write) -> std::io::Result<()> {
    out.write_all(b"hello world\n")?;
    out.flush()
}

use std::fs::File;

fn exec() -> std::io::Result<()> {
    let mut local_file = File::create("hello.txt")?;
    say_hello(&mut local_file)?;

    let mut bytes = vec![];
    say_hello(&mut bytes)?;
    
    assert_eq!(bytes, b"hello world\n");

    Ok(())
}

fn main() {
    exec().unwrap();
}
