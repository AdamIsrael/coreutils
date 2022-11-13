use clap::Parser;

use tabled::locator::ByColumnName;
use tabled::object::{Columns, Object, Rows, Segment};
use tabled::{Alignment, Disable, ModifyObject, Style, Table, Tabled};

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

#[derive(Debug, Tabled)]
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

    if args.files.is_empty() {
        // read from stdin
        let mut stat = FileStats {
            chars: 0,
            bytes: 0,
            lines: 0,
            words: 0,
            filename: "".to_string(),
        };

        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let buf = line.unwrap();

            // Tally the stats
            count_all(&buf, &mut stat);
        }
        stats.push(stat);
    } else {
        for filename in &args.files {
            let file = match File::open(filename) {
                Err(why) => panic!("couldn't open: {}", why),
                Ok(file) => file,
            };

            let mut stat = FileStats {
                chars: 0,
                bytes: 0,
                lines: 0,
                words: 0,
                filename: filename.to_string(),
            };
            let mut reader = io::BufReader::new(file);
            let mut buf = String::new();
            while reader.read_line(&mut buf).unwrap() > 0 {
                // Tally the stats
                count_all(&buf, &mut stat);

                // clear the buffer for the next read
                buf.clear();
            }
            stats.push(stat);
        }

        // If there are more than one file, display a total row
        if args.files.len() > 1 {
            // Generate the _totals_ and add it to stats
            let mut total = FileStats {
                chars: 0,
                bytes: 0,
                lines: 0,
                words: 0,
                filename: "total".to_string(),
            };
            for stat in &stats {
                total.chars += stat.chars;
                total.bytes += stat.bytes;
                total.lines += stat.lines;
                total.words += stat.words;
            }
            stats.push(total);
        }
    }

    // Display the output
    let mut table = Table::new(&stats);

    // If all args are false, display the whole table.
    if !args.cbytes && !args.lines && !args.mchars && !args.words {
        // By default, don't show characters
        table.with(Disable::column(ByColumnName::new("chars")));
    } else {
        // Disable columns based on argument
        if !args.cbytes {
            table.with(Disable::column(ByColumnName::new("bytes")));
        }
        if !args.lines {
            table.with(Disable::column(ByColumnName::new("lines")));
        }
        if !args.words {
            table.with(Disable::column(ByColumnName::new("words")));
        }
        if !args.mchars {
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

    println!("{}", table);
}

fn count_all(buf: &str, stat: &mut FileStats) {
    // increment the byte count
    stat.bytes += buf.len() as i32;

    // increment the _character_ count
    stat.chars += buf.chars().count() as i32;

    // Get the line, and trim the newline
    let line = buf.trim_end();

    // increment the line count
    stat.lines += 1;

    // increment the word count
    let words = count_words(line);
    stat.words += words;
}

fn count_words(text: &str) -> i32 {
    let mut words = 0;
    for word in text.split(|c: char| c.is_whitespace()) {
        if !word.trim().is_empty() {
            words += 1;
        }
    }
    words
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs::File;
    use std::io;

    #[test]
    fn test_wc() {
        let mut stats = FileStats {
            chars: 0,
            bytes: 0,
            lines: 0,
            words: 0,
            filename: "total".to_string(),
        };
        let filename = "../data/ipso_old_english.txt".to_string();
        let file = match File::open(&filename) {
            Err(why) => panic!("couldn't open: {}", why),
            Ok(file) => file,
        };
        let mut reader = io::BufReader::new(file);
        let mut buf = String::new();
        while reader.read_line(&mut buf).unwrap() > 0 {
            // Tally the stats
            count_all(&buf, &mut stats);

            // clear the buffer for the next read
            buf.clear();
        }

        // Assert that we got the stats we were expecting
        assert_eq!(stats.bytes, 1783);
        assert_eq!(stats.chars, 1575);
        assert_eq!(stats.lines, 9);
        assert_eq!(stats.words, 253);
    }
}
