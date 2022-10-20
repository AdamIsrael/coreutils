use std::fs::File;
use std::io;
use std::io::prelude::*;

use base64ct::{Base64, Encoding};
use blake2::{Blake2b512, Digest};
use clap::Parser;
use hex_literal::hex;

/// Print or check BLAKE2 (512-bit) checksums.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// read in binary mode
    #[arg(short)]
    binary: bool,

    /// read BLAKE2 sums from the FILEs and check them
    #[arg(short)]
    check: bool,

    files: Vec<String>,

    /// don't fail or report status for missing files
    #[arg(long)]
    ignore_missing: bool,

    /// digest length in bits; must not exceed the maximum for the blake2 algorithm and must be a multiple of 8
    #[arg(short)]
    length: bool,

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

    /// read in text mode (default)
    #[arg(short)]
    text: bool,

    /// warn about improperly formatted files
    #[arg(short)]
    warn: bool,

    /// end each output line with NUL, not newline, and disable file name escaping
    #[arg(short)]
    zero: bool,
}

fn main() {
    let args = Args::parse();
    let checksum = run(args);

    println!("{}", checksum);
}

fn run(args: Args) -> String {
    if !args.files.is_empty() {
    } else {
        // read from stdin
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let buf = line.unwrap();
            println!("Buf: {:?}", buf);
            // let mut hasher = Blake2b512::new();
            // hasher.update(buf.as_bytes());

            // let res = hasher.finalize();
            // println!("{:?}", res);

            let res = b2sum(buf);
            // print the hex hash
            println!("Res: {:?}", res);

            // assert_eq!(
            //     res[..],
            //     hex!(
            //         "
            //     021ced8799296ceca557832ab941a50b4a11f83478cf141f51f933f653ab9fbc
            //     c05a037cddbed06e309bf334942c4e58cdf1a46e237911ccd7fcf9787cbc7fd0
            // "
            //     )[..]
            // );

            // let hash = Black2b::Digest(res);
            // let mut buf = [0u8; 16];
            // let hex_hash = base16ct::lower::encode_str(&res, &mut buf);
            // let hex_hash = hex!(res);
            // println!("Hex-encoded hash: {:?}", hex_hash);
        }
    }
    return String::from("foo");
}

fn b2sum(buf: String) -> String {
    let mut hasher = Blake2b512::new();
    hasher.update(buf);

    let res = hasher.finalize();

    return format!("{:x}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_b2sum_hello() {
        let hash = b2sum(String::from("hello"));
        assert_eq!(
            &hash,
            "e4cfa39a3d37be31c59609e807970799caa68a19bfaa15135f165085e01d41a65ba1e1b146aeb6bd0092b49eac214c103ccfa3a365954bbbe52f74a2b3620c94"
        );
    }

    #[test]
    fn test_b2sum_hello_world() {
        let hash = b2sum(String::from("hello, world"));
        assert_eq!(
            &hash,
            "7355dd5276c21cfe0c593b5063b96af3f96a454b33216f58314f44c3ade92e9cd6cec4210a0836246780e9baf927cc50b9a3d7073e8f9bd12780fddbcb930c6d"
        );
    }

    // #[test]
    // fn test_architecture() {

    //     let args = Args {
    //         binary: false,
    //         check: false,
    //         files: vec![""],
    //         ignore_missing: false,
    //         length: false,
    //         missing: false,
    //         status: false,
    //         strict: false,
    //         tag: false,
    //         warn: false,
    //         zero: false,
    //     };

    //     let basenames = run(args);
    //     assert_eq!(basenames.len(), 1);

    // }
}
