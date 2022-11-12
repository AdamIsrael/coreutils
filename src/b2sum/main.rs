use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::ErrorKind;
use std::io::Write;
use std::process;

use blake2::{Blake2b512, Digest};
use clap::Parser;
// use exitcode;

/// Print or check BLAKE2 (512-bit) checksums.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // // There's no difference between --binary and --text on GNU systems, so I'm not
    // // sure how to implement and test this.
    // /// read in binary mode
    // #[arg(short, long)]
    // binary: bool,
    /// read BLAKE2 sums from the FILEs and check them
    #[arg(short, long)]
    check: bool,

    files: Vec<String>,

    /// don't fail or report status for missing files
    #[arg(long)]
    ignore_missing: bool,

    /// digest length in bits; must not exceed the maximum for the blake2 algorithm and must be a multiple of 8
    #[arg(short, long, default_value_t = 128)]
    length: i32,

    /// don't print OK for each successfully verified file
    #[arg(long)]
    quiet: bool,

    /// don't output anything, status code shows success
    #[arg(long)]
    status: bool,

    /// exit non-zero for improperly formatted checksum lines
    #[arg(long)]
    strict: bool,

    /// create a BSD-style checksum
    #[arg(long)]
    tag: bool,

    // /// read in text mode (default)
    // #[arg(short, long, default_value_t = true)]
    // text: bool,
    /// warn about improperly formatted files
    #[arg(short, long)]
    warn: bool,

    /// end each output line with NUL, not newline, and disable file name escaping
    #[arg(short, long)]
    zero: bool,
}

struct B2Hash {
    filename: String,
    hash: String,
}

fn main() {
    let mut retcode = 0;
    let args = Args::parse();

    if args.check {
        retcode = check(&args);
    } else {
        let checksums = run(&args);

        for checksum in checksums {
            if args.length == 0 {
                output_hash(&args, checksum.hash, checksum.filename);
            } else if args.length % 8 == 0 {
                // length must be a multiple of 8
                let slice = &checksum.hash[..args.length as usize];

                output_hash(&args, slice.to_string(), checksum.filename);
            } else {
                output(
                    &args,
                    format!("length ({}) is not a multiple of 8", args.length),
                );
            }
        }
    }
    process::exit(retcode);
}

/// Print the output of a successful hash
fn output_hash(args: &Args, hash: String, filename: String) {
    if args.tag {
        output(
            args,
            format!("BLAKE2b-{} ({}) = {}", args.length, filename, hash),
        );
    } else {
        output(args, format!("{} {}", hash, filename));
    }
}

/// Output the line with either a newline or NUL
fn output(args: &Args, line: String) {
    if args.zero {
        print!("{}\0", line);
        io::stdout().flush().unwrap();
    } else {
        println!("{}", line);
    }
}

/// Perform the checksum validation
fn check(args: &Args) -> i32 {
    /*
    read from the file, which will be in the following format (one per line):
    <b2sum hash> <filename>
    then hash the filename and compare it to the hash in the file. If they're okay, print:
    <filename>: OK
    if the hashes don't match, print:
    <filename>: FAILED
    b2sum: WARNING: <n> computed checksum did NOT match
    */
    let mut retval = 0;
    let mut failed = 0;

    for filename in &args.files {
        let file = match File::open(filename) {
            Err(why) => panic!("couldn't open: {}", why),
            Ok(file) => file,
        };

        let mut reader = io::BufReader::new(file);
        let mut buf = String::new();
        while reader.read_line(&mut buf).unwrap() > 0 {
            let mut iter = buf.split_whitespace();

            let hash = match iter.next() {
                Some(hash) => hash,
                None => todo!(),
            };
            let fname = match iter.next() {
                Some(fname) => fname,
                None => todo!(),
            };

            // If there's anything else in the iterator, something's wrong
            // with the file.
            if iter.next().is_some() {
                if args.warn {
                    println!("b2sum: WARNING: Listed file could not be read");

                    // Skip to the next line
                    buf.clear();
                    continue;
                } else {
                    panic!("Invalid file format.");
                }
            }

            let hash2 = match b2sum_file(fname.to_string()) {
                Err(why) => {
                    if args.ignore_missing {
                        // skip this file
                        buf.clear();
                        continue;
                    } else {
                        output(args, format!("b2sum: {}: {}", fname, why));
                    }
                    "".to_string()
                }
                Ok(hash) => hash,
            };

            // TODO: return this information to the caller, so main() can
            // process it and handle returning the right error code.
            if hash == hash2 {
                if !args.quiet && !args.status {
                    output(args, format!("{}: OK", fname));
                }
            } else {
                if !args.quiet && !args.status {
                    output(args, format!("{}: FAILED", fname));
                }

                failed += 1;
            }

            // clear the buffer for the next read
            buf.clear();
        }
    }
    if failed > 0 {
        retval = 1;
        if !args.status {
            output(
                args,
                format!("b2sum: WARNING: {} computed checksum did NOT match", failed),
            );
        }
    }
    retval
}

/// Generate a checksum for the specified input
fn run(args: &Args) -> Vec<B2Hash> {
    let mut retval = Vec::new();

    if !args.files.is_empty() {
        for filename in &args.files {
            let hash = match b2sum_file(filename.to_string()) {
                Err(why) => {
                    if args.ignore_missing {
                        // skip this file
                        continue;
                    } else {
                        output(args, format!("b2sum: {}: {}", filename, why));
                    }
                    "".to_string()
                }
                Ok(hash) => hash,
            };
            retval.push(B2Hash {
                hash,
                filename: filename.to_string(),
            });
        }
    } else {
        // read from stdin
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let buf = line.unwrap();

            // Get the result of the b2sum hash
            let res = b2sum_string(buf);

            retval.push(B2Hash {
                hash: res.to_string(),
                filename: "-".to_string(),
            });
        }
    }

    retval
}

/// Get the b2sum of a file
/// Returns a Result<String, ErrorKind> so that we can surface file errors like
/// file not found or lack of read permissions.
fn b2sum_file(filename: String) -> Result<String, ErrorKind> {
    let file = match File::open(&filename) {
        Err(why) => {
            return Err(why.kind());
        }
        Ok(file) => file,
    };

    let mut hasher = Blake2b512::new();
    let mut reader = io::BufReader::new(file);
    let mut buf = String::new();
    while reader.read_line(&mut buf).unwrap() > 0 {
        // Update the hasher with the next line in the file
        hasher.update(&buf);

        // clear the buffer for the next read
        buf.clear();
    }
    let res = hasher.finalize();

    Ok(format!("{:x}", res))
}

// fn b2sum_file(filename: String) -> String {
//     let file = match File::open(&filename) {
//         Err(why) => panic!("couldn't open: {}", why),
//         Ok(file) => file,
//     };

//     let mut hasher = Blake2b512::new();
//     let mut reader = io::BufReader::new(file);
//     let mut buf = String::new();
//     while reader.read_line(&mut buf).unwrap() > 0 {
//         // Update the hasher with the next line in the file
//         hasher.update(&buf);

//         // clear the buffer for the next read
//         buf.clear();
//     }
//     let res = hasher.finalize();

//     format!("{:x}", res)
// }

/// Get the b2sum of a string
fn b2sum_string(buf: String) -> String {
    let mut hasher = Blake2b512::new();
    hasher.update(buf);

    let res = hasher.finalize();

    format!("{:x}", res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_b2sum_hello() {
        let hash = b2sum_string(String::from("hello"));
        assert_eq!(
            &hash,
            "e4cfa39a3d37be31c59609e807970799caa68a19bfaa15135f165085e01d41a65ba1e1b146aeb6bd0092b49eac214c103ccfa3a365954bbbe52f74a2b3620c94"
        );
    }

    #[test]
    fn test_b2sum_hello_world() {
        let hash = b2sum_string(String::from("hello, world"));
        assert_eq!(
            &hash,
            "7355dd5276c21cfe0c593b5063b96af3f96a454b33216f58314f44c3ade92e9cd6cec4210a0836246780e9baf927cc50b9a3d7073e8f9bd12780fddbcb930c6d"
        );
    }
}
