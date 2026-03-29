use std::fs::read_to_string;
use std::io;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use std::process;
use std::str;

use clap::{Arg, ArgAction, arg};
use serde_json::json;
use tabled::{builder::Builder, settings::Style, settings::Width};

use coreutils::{clap_args, clap_base_command};

// const ALPHABET: base64::Alphabet = base64::Alphabet::RFC4648 { padding: false };
const DEFAULT_WRAP: i32 = 76;

clap_args!(Args {
    flag decode: bool,
    flag ignore_garbage: bool,
    value(DEFAULT_WRAP) wrap: i32,
    maybe file: Option<String>,
});

fn main() {
    let matches = clap_base_command()
        .arg(arg!(-d --decode "decode data"))
        .arg(
            Arg::new("ignore_garbage")
                .long("ignore-garbage")
                .action(ArgAction::SetTrue)
                .help("ignore non-alphabet characters when decoding"),
        )
        .arg(arg!(-w --wrap <LENGTH> "wrap output lines after LENGTH characters (plain, table)"))
        .arg(
            Arg::new("file")
                .action(ArgAction::Set)
                .help("the name of the file to read from"),
        )
        .get_matches();

    let args = Args::from_matches(&matches);

    let retval = run(&args);

    process::exit(retval);
}

/// Compute the base64 hash of the input data
fn compute(args: &Args) -> Result<String, Error> {
    if let Some(ref file) = args.file {
        return base64_file(args, file);
    }
    let mut buf = String::new();
    io::stdin().lock().read_to_string(&mut buf).unwrap();
    buf = buf.trim().to_string();
    if args.decode {
        remove_newlines(&mut buf);
        if args.ignore_garbage {
            ignore_garbage(&mut buf);
        }
        decode_base64_string(&buf)
    } else {
        Ok(encode_base64_string(&buf))
    }
}

/// Run the base64 command with the given arguments
fn run(args: &Args) -> i32 {
    match compute(args) {
        Ok(hash) => {
            if let Some(output) = &args.output {
                match output.as_str() {
                    "table" => {
                        let mut builder = Builder::new();
                        builder.push_column(["base64"]);
                        builder.push_record([hash]);
                        let mut table = builder.build();
                        println!(
                            "{}",
                            table
                                .with(Style::rounded())
                                .with(Width::wrap(get_wrap(args) as usize))
                        );
                    }
                    "json" => {
                        let output = json!({
                            "base64": hash,
                        });
                        println!("{}", serde_json::to_string(&output).unwrap());
                    }
                    "yaml" => println!("base64: \"{hash}\""),
                    _ => println!("{}", wrap(args, &hash)),
                }
            }
            0
        }
        Err(why) => {
            eprintln!("{}", why);
            1
        }
    }
}

/// Ignore non-alphabet characters
fn ignore_garbage(s: &mut String) {
    *s = str::replace(s, |c: char| !c.is_alphanumeric() && c != '=', "");
}

/// Remove newlines embedded within the string, most likely from line wrapping.
fn remove_newlines(s: &mut String) {
    s.retain(|c| c != '\n');
}

fn get_wrap(args: &Args) -> i32 {
    if args.wrap > 0 {
        args.wrap
    } else {
        DEFAULT_WRAP
    }
}

/// Wrap the hash into multiple lines
fn wrap(args: &Args, data: &str) -> String {
    // https://users.rust-lang.org/t/solved-how-to-split-string-into-multiple-sub-strings-with-given-length/10542/3
    let lines = data
        .as_bytes()
        .chunks(get_wrap(args) as usize)
        .map(str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap();

    lines.join("\n")
}

/// Get the base64 of a file
fn base64_file(args: &Args, filename: &str) -> Result<String, Error> {
    let buf = match read_to_string(filename) {
        Err(why) => {
            let err_not_found = Error::new(
                ErrorKind::NotFound,
                format!("base64: '{}': {}", filename, why),
            );
            return Err(err_not_found);
        }
        Ok(buf) => buf.trim().to_string(),
    };

    let data: String = if args.decode {
        decode_base64_string(&buf)?
    } else {
        encode_base64_string(&buf)
    };

    Ok(data)
}

/// Get the base64 of a String
fn encode_base64_string(str: &str) -> String {
    base64::encode(str.as_bytes())
}

/// Decode a base64 string into a String
fn decode_base64_string(str: &str) -> Result<String, Error> {
    let buf = match base64::decode(str) {
        Err(why) => {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("base64: {}", why),
            ));
        }
        Ok(buf) => buf,
    };

    // after we've stripped garbage from a string, this might fail so we need
    // error checking
    let hash = match str::from_utf8(&buf) {
        Err(why) => {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("base64: {}", why),
            ));
        }
        Ok(hash) => hash,
    };

    Ok(hash.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64() {
        let hello = String::from("hello, world");
        let hash = encode_base64_string(&hello);
        assert_eq!(hello, decode_base64_string(&hash).unwrap());
    }

    #[test]
    fn test_ignore_garbage() {
        let mut input = String::from(
            "aGVsbG8sI
        Hdvcmxk",
        );

        ignore_garbage(&mut input);
        assert_eq!("hello, world", decode_base64_string(&input).unwrap());
    }
}
