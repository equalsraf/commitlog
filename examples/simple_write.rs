extern crate commitlog;

use commitlog::*;

fn main() {
    let opts = LogOptions::new("log");
    let mut log = CommitLog::new(opts).unwrap();

    // append to the log
    log.append(b"hello world").unwrap(); // offset 0
    log.append(b"second message").unwrap(); // offset 1

    // read the messages
    let messages = log.read(ReadPosition::Beginning, ReadLimit::Messages(2)).unwrap();
    for msg in messages {
        println!("{} - {}", msg.offset(), String::from_utf8_lossy(msg.payload()));
    }

    // prints:
    //    0 - hello world
    //    1 - second message
}