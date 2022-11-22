use std::fs::File;
use std::io;
use std::io::prelude::*;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// equivalent to -vET
    #[arg(short = 'A', long)]
    show_all: bool,

    /// number nonempty output lines, overrides -n
    #[arg(short = 'b', long)]
    number_nonblank: bool,

    /// equivalent to -vE
    #[arg(short = 'e')]
    e: bool,

    /// display $ at the end of each line
    #[arg(short = 'E', long)]
    show_ends: bool,

    /// number all output lines
    #[arg(short = 'n', long)]
    number: bool,

    /// suppress repeated empty output lines
    #[arg(short, long)]
    squeeze_blank: bool,

    /// equivalent to -vT
    #[arg(short = 't')]
    t: bool,

    /// display TAB characters as ^I
    #[arg(short = 'T', long)]
    show_tabs: bool,

    /// ignored
    #[arg(short)]
    u: bool,

    /// use ^ and M- notation, except for LFD and  TAB
    #[arg(short = 'v', long)]
    show_nonprinting: bool,

    files: Vec<String>,
}
fn main() {
    let args = Args::parse();

    if args.files.is_empty() {
        // read from stdin
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let buf = line.unwrap();

            // print to stdout
            // println!("{buf}");
            output(&args, &buf);
        }
    } else {
        for filename in &args.files {
            let file = match File::open(filename) {
                Err(why) => panic!("couldn't open: {}", why),
                Ok(file) => file,
            };

            let mut reader = io::BufReader::new(file);
            let mut buf = String::new();
            while reader.read_line(&mut buf).unwrap() > 0 {
                // println!("{}", buf.trim_end());
                output(&args, &buf);

                // clear the buffer for the next read
                buf.clear();
            }
        }
    }
}

/// Output the string according to stdargs
fn output(args: &Args, line: &str) {
    let mut s = line.to_owned();

    // strip the line ending; we'll add our own
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }

    // display $ at the end of each line
    if args.show_ends {
        s += "$";
    }

    // show tabs
    if args.show_tabs {
        s = s.replace('\t', "^I");
    }

    println!("{}", s);
}
