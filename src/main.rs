use std::io;
use std::collections::HashMap;


fn main() {
    let mut counts: HashMap<String, usize> = HashMap::new();

    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => {
                let count = counts.entry(input.to_string()).or_insert(0);
                *count += 1;
            }
            Err(_) => break,
        }
    }
    for (key, value) in counts.iter() {
        println!("{:7} {}", value, key.trim());
    }
}
