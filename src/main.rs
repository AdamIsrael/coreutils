use std::collections::HashMap;
use std::env;

use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() {
    let filename = env::args().nth(1);
    let mut map: HashMap<String, i32> = HashMap::new();
    let mut lc = 0;

    // check input: a file or stdin?
    match filename {
        None => {
            // There's no filename, so try reading from stdin
            let stdin = io::stdin();
            for line in stdin.lock().lines() {
                lc += 1;
                count_words(&mut map, &line.unwrap());
            }
        }
        Some(f) => {
            let file = match File::open(&f) {
                Err(why) => panic!("couldn't open: {}", why),
                Ok(file) => file,
            };

            let mut reader = io::BufReader::new(file);
            let mut buf = String::new();
            while reader.read_line(&mut buf).unwrap() > 0 {
                {
                    // Get the line, and trim the newline
                    let line = buf.trim_end();
                    lc += 1;
                    count_words(&mut map, line);
                }
                // clear the buffer for the next read
                buf.clear();
            }
        }
    }
    println!("word count: {}", map.len());
    println!("line count: {}\n", lc);

    // // Find the top used words
    // let mut entries: Vec<_> = map.into_iter().collect();
    // entries.sort_by(|a, b| b.1.cmp(&a.1));
    // for e in entries.iter().take(10) {
    //     println!("{} {}", e.0, e.1);
    // }
}

fn count_words(map: &mut HashMap<String, i32>, text: &str) {
    for s in text.split(|c: char| !c.is_alphabetic()) {
        let word = s.to_lowercase();

        // Skip punctuation or extra spaces
        if word.trim().len() > 0 {
            let c = map.entry(word).or_insert(0);
            *c += 1;
        }
    }
}
