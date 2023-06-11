use std::fs::read_to_string;
use std::io;
use std::io::prelude::*;
use std::io::ErrorKind;
use std::process;
use std::str;

use base64::{decode, encode};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// decode data
    #[arg(short, long)]
    decode: bool,

    /// when decoding, ignore non-alphabet characters
    #[arg(short, long)]
    ignore_garbage: bool,

    #[arg(short, long, default_value_t = 76)]
    wrap: i32,

    /// accept a single filename
    #[clap(default_value_t)]
    file: String,
}

fn main() {
    let args = Args::parse();

    let retval = run(&args);

    process::exit(retval);
}

fn run(args: &Args) -> i32 {
    let retval = 0;

    if !args.file.is_empty() {
        let hash = match base64_file(args, args.file.to_string()) {
            Err(why) => {
                println!("base64: {why}");
                return 1;
            }
            Ok(hash) => hash,
        };
        println!("{hash}");
    } else {
        let stdin = io::stdin();
        let mut buf = String::new();

        // Slurp the data from stdin
        stdin.lock().read_to_string(&mut buf).unwrap();

        // Trim the whitespace. We've got a trailing newline
        buf = buf.trim().to_string();

        if args.decode {
            // Remove the newlines from the wrapped string
            remove_newlines(&mut buf);
            if args.ignore_garbage {
                ignore_garbage(&mut buf);
            }

            let data = decode_base64_string(&buf);

            println!("{data}");
        } else {
            output(args, encode_base64_string(&buf));
        }
    }

    retval
}

/// Ignore non-alphabet characters
fn ignore_garbage(s: &mut String) {
    *s = str::replace(s, |c: char| !c.is_alphanumeric(), "");
    // *s = str::replace(s, |c: char| !c.is_alphanumeric() && c != '=', "");
}

/// Remove newlines embedded within the string, most likely from line wrapping.
fn remove_newlines(s: &mut String) {
    s.retain(|c| c != '\n');
}

/// Output the string with wrapping
fn output(args: &Args, data: String) {
    // https://users.rust-lang.org/t/solved-how-to-split-string-into-multiple-sub-strings-with-given-length/10542/3
    let lines = data
        .as_bytes()
        .chunks(args.wrap as usize)
        .map(str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap();

    for line in lines {
        println!("{line}");
    }
}

/// Get the base64 of a file
fn base64_file(args: &Args, filename: String) -> Result<String, ErrorKind> {
    let buf = match read_to_string(filename) {
        Err(why) => {
            return Err(why.kind());
        }
        Ok(buf) => buf.trim().to_string(),
    };

    let data: String = if args.decode {
        decode_base64_string(&buf)
    } else {
        encode_base64_string(&buf)
    };

    Ok(data)
}

// fn get_alphabet() -> base64::Alphabet {
//     let alpha: base64::Alphabet = base64::Alphabet::RFC4648 { padding: false };

//     alpha
// }

// Get the base64 of a String
fn encode_base64_string(str: &String) -> String {
    // let alpha = get_alphabet();
    // base64::encode(alpha, str.as_bytes())
    encode(str)
}

fn decode_base64_string(str: &String) -> String {
    let buf = &decode(str).unwrap()[..];

    match str::from_utf8(buf) {
        Ok(v) => v.to_string(),
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64() {
        let hello = String::from("hello, world");
        let hash = encode_base64_string(&hello);
        assert_eq!(hello, decode_base64_string(&hash));
    }

    #[test]
    fn test_ignore_garbage() {
        let mut input = String::from(
            "aGVsbG8s
        IHdvcmxk
        ",
        );

        ignore_garbage(&mut input);
        println!("input: '{}'", input);
        assert_eq!("hello, world", decode_base64_string(&input));
    }
}
