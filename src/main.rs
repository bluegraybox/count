// This counts the number of occurrences of each unique line of input,
// and outputs each prefixed by its count.
// Like `sort | uniq -c`.

use std::collections::HashMap;
use std::io;

fn main() {
    let mut counts: HashMap<String, usize> = HashMap::new();

    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => break,
            Ok(_bytes_read) => {
                let key = input.to_string();
                let count = counts.entry(key).or_insert(0);
                *count += 1;
            }
            Err(msg) => {
                println!("ERROR: {}", msg);
                break;
            }
        }
    }
    for (key, value) in counts.iter() {
        println!("{:7} {}", value, key.trim());
    }
}
