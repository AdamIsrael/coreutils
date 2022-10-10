use clap::Parser;
use std::collections::HashMap;

use tabled::{Alignment, Disable, ModifyObject, Style, Table, Tabled};
use tabled::object::{Columns, Object, Rows, Segment};
use tabled::locator::ByColumnName;

use std::fs::File;
use std::io;
use std::io::prelude::*;

/// A rust implementation of wc: word, line, character, and byte count.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // wc [-clmw] [file ...]
    /// The number of bytes in each input file is written to the standard output.
    #[arg(short)]
    cbytes: bool,

    /// The number of lines in each input file is written to the standard output.
    #[arg(short)]
    lines: bool,

    /// The number of characters in each input file is written to the standard output.
    #[arg(short)]
    mchars: bool,

    /// The number of words in each input file is written to the standard output.
    #[arg(short)]
    words: bool,

    files: Vec<String>,
}

#[derive(Tabled)]
struct FileStats {
    lines: i32,
    words: i32,
    chars: i32,
    bytes: i32,
    filename: String,
}

fn main() {
    let args = Args::parse();
    // println!("{:?}", args);

    let mut stats = Vec::<FileStats>::new();

    // A hashmap of filenames and metric
    let mut bc: HashMap<String, i32> = HashMap::new(); // byte count
    let mut cc: HashMap<String, i32> = HashMap::new(); // character count
    let mut lc: HashMap<String, i32> = HashMap::new(); // line count
    let mut wc: HashMap<String, i32> = HashMap::new(); // word count

    if args.files.len() == 0 {
        // read from stdin
        println!("TODO: read from stdin");

        // let stdin = io::stdin();
        // for line in stdin.lock().lines() {
        // }
    } else {
        for filename in &args.files {
            let file = match File::open(&filename) {
                Err(why) => panic!("couldn't open: {}", why),
                Ok(file) => file,
            };
            let mut reader = io::BufReader::new(file);
            let mut buf = String::new();
            while reader.read_line(&mut buf).unwrap() > 0 {
                {
                    // increment the byte count
                    let c = bc.entry(filename.to_string()).or_insert(0);
                    *c += buf.len() as i32;

                    // increment the _character_ count
                    let chars: Vec<char> = buf.chars().collect();
                    let c = cc.entry(filename.to_string()).or_insert(0);
                    *c += chars.len() as i32;

                    // Get the line, and trim the newline
                    let line = buf.trim_end();

                    // increment the line count
                    let c = lc.entry(filename.to_string()).or_insert(0);
                    *c += 1;

                    // increment the word count
                    let words = count_words(line);
                    let c = wc.entry(filename.to_string()).or_insert(0);
                    *c += words;
                }
                // clear the buffer for the next read
                buf.clear();
            }

            let stat = FileStats {
                chars: *cc.get(&filename.to_string()).unwrap(),
                bytes: *bc.get(&filename.to_string()).unwrap(),
                lines: *lc.get(&filename.to_string()).unwrap(),
                words: *wc.get(&filename.to_string()).unwrap(),
                filename: filename.to_string(),
            };

            stats.push(stat);
        }

        // If there are more than one file, display a table of results
        if args.files.len() > 1 {
            // Generate the _totals_ and add it to stats
            let total = FileStats {
                chars: 0,
                bytes: bc.values().sum::<i32>(),
                lines: lc.values().sum::<i32>(),
                words: wc.values().sum::<i32>(),
                filename: "total".to_string(),
            };
            stats.push(total);

            let mut builder = Table::builder(&stats);

            // TODO: check args and remove column(s)
            let mut table = builder.build();

            // If all args are false, display the whole table.
            if args.cbytes == false
                && args.lines == false
                && args.mchars == false
                && args.words == false
            {
                // By default, don't show characters
                table.with(Disable::column(ByColumnName::new("chars")));

            } else {
                // Disable columns based on argument
                if args.cbytes == false {
                    table.with(Disable::column(ByColumnName::new("bytes")));
                }
                if args.lines == false {
                    table.with(Disable::column(ByColumnName::new("lines")));
                }
                if args.words == false {
                    table.with(Disable::column(ByColumnName::new("words")));
                }
                if args.mchars == false {
                    table.with(Disable::column(ByColumnName::new("chars")));
                }
            }

            // Remove header row before styling the table
            table.with(Disable::row(Rows::first()));

            // Stylize the table
            table
                .with(Style::blank())
                // Right align everything but the filename column
                .with(
                    Segment::all()
                        .not(Columns::last())
                        .modify()
                        .with(Alignment::right()),
                );

            println!("{}", table.to_string());
        }
    }
}

fn count_words(text: &str) -> i32 {
    let mut words = 0;
    for word in text.split(|c: char| c.is_whitespace()) {
        if word.trim().len() > 0 {
            words += 1;
        }
    }
    words
}
