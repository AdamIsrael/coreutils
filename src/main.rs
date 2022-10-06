use clap::Parser;

use std::collections::HashMap;
use std::env;

use std::fs::File;
use std::io;
use std::io::prelude::*;

/// A rust implementation of wc: word, line, character, and byte count.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // wc [-clmw] [file ...]

    /// The number of bytes
    #[arg(short)]
    cbytes: bool,

    /// Count the number of lines...
    #[arg(short)]
    lines: bool,

    /// Count the characters...
    #[arg(short)]
    mchars: bool,

    /// Count the words...
    #[arg(short)]
    words: bool,

    files: Vec<std::path::PathBuf>,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);

    // Why not? See if we can make a vector of hashmaps
    // it works, in that I can create the vec, but accessing the
    // hashmap seems to be problematic, in passing to count_words.
    //let maps: Vec<HashMap<String, i32>> = vec![HashMap::new()];

    // What about a hashmap of hashmaps?
    let mut maps: HashMap<String, HashMap<String, i32>>;

    let mut lc = 0;
    // initialize the first HashMap; there will always be 1 minimum
    // maps.push(HashMap::new());

    if args.files.len() == 0 {
        // read from stdin
        println!("No files.");
    } else {
        for filename in &args.files {
            println!("{:?}", filename);

            // create the hashmap for the filename
            let map = maps.entry(filename.to_string()).or_insert(HashMap::new());

            let file = match File::open(&filename) {
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

                    // let mut map = &maps[0];
                    // count_words(&mut &maps[0], line);
                }
                // clear the buffer for the next read
                buf.clear();
            }

        }
        println!("File 0: {:?}", args.files.get(0));
    }

    // everything below here works
    let filename = env::args().nth(1);
    let mut map: HashMap<String, i32> = HashMap::new();

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
