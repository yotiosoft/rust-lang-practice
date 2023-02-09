use std::io;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;
use std::{thread, time};

fn main() {
    let stdin_channel = spawn_stdin_channel();
    let mut buffer = Vec::new();
    let mut flag = 0;
    let mut flag_total = 0;
    loop {
        match stdin_channel.try_recv() {
            Ok(key) => {
                //println!("{} key length: {}", key, key.len());
                buffer.push(key);
                flag += 1;
                flag_total += 1;
            },
            Err(TryRecvError::Empty) => {
                if flag == 0 && flag_total > 0 {
                    println!("input: {}", buffer.join(""));
                    buffer.clear();
                    flag_total = 0;
                }

                flag = 0;
            },//println!("Channel empty"),
            Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
        }
        sleep(1);
    }
}

fn spawn_stdin_channel() -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        tx.send(buffer).unwrap();
    });
    rx
}

fn sleep(millis: u64) {
    let duration = time::Duration::from_millis(millis);
    thread::sleep(duration);
}
