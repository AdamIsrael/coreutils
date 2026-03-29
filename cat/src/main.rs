use std::io;
use std::io::prelude::*;
use std::process;

use clap::{Arg, ArgAction};

use coreutils::{clap_args, clap_base_command};

clap_args!(Args {
    flag show_all: bool,
    flag number_nonblank: bool,
    flag e: bool,
    flag show_ends: bool,
    flag number: bool,
    flag squeeze_blank: bool,
    flag t: bool,
    flag show_tabs: bool,
    flag u: bool,
    flag show_nonprinting: bool,
    multi files: Vec<String>,
});

impl Args {
    fn tab(&self) -> &'static str {
        if self.show_tabs { "^I" } else { "\t" }
    }
}

fn main() {
    let matches = clap_base_command()
        .arg(
            Arg::new("show_all")
                .short('A')
                .long("show-all")
                .action(ArgAction::SetTrue)
                .help("equivalent to -vET"),
        )
        .arg(
            Arg::new("number_nonblank")
                .short('b')
                .long("number-nonblank")
                .action(ArgAction::SetTrue)
                .help("number nonempty output lines, overrides -n"),
        )
        .arg(
            Arg::new("e")
                .short('e')
                .action(ArgAction::SetTrue)
                .help("equivalent to -vE"),
        )
        .arg(
            Arg::new("show_ends")
                .short('E')
                .long("show-ends")
                .action(ArgAction::SetTrue)
                .help("display $ at the end of each line"),
        )
        .arg(
            Arg::new("number")
                .short('n')
                .long("number")
                .action(ArgAction::SetTrue)
                .help("number all output lines"),
        )
        .arg(
            Arg::new("squeeze_blank")
                .short('s')
                .long("squeeze-blank")
                .action(ArgAction::SetTrue)
                .help("suppress repeated empty output lines"),
        )
        .arg(
            Arg::new("t")
                .short('t')
                .action(ArgAction::SetTrue)
                .help("equivalent to -vT"),
        )
        .arg(
            Arg::new("show_tabs")
                .short('T')
                .long("show-tabs")
                .action(ArgAction::SetTrue)
                .help("display TAB characters as ^I"),
        )
        .arg(
            Arg::new("u")
                .short('u')
                .action(ArgAction::SetTrue)
                .help("ignored"),
        )
        .arg(
            Arg::new("show_nonprinting")
                .short('v')
                .long("show-nonprinting")
                .action(ArgAction::SetTrue)
                .help("use ^ and M- notation, except for LFD and  TAB"),
        )
        .arg(
            Arg::new("files")
                .action(ArgAction::Append)
                .help("the file(s) to concatenate"),
        )
        .get_matches();

    let mut args = Args::from_matches(&matches);

    // do shortcut: -A
    if args.show_all {
        args.show_nonprinting = true;
        args.show_ends = true;
        args.show_tabs = true;
    }

    // do shortcut: -e
    if args.e {
        args.show_nonprinting = true;
        args.show_ends = true;
    }

    // do shortcut: -t
    if args.t {
        args.show_nonprinting = true;
        args.show_tabs = true;
    }

    let mut retval = 0;
    let mut line_count: usize = 1;
    let mut stdout = io::stdout().lock();
    let mut stderr = io::stderr().lock();

    if args.files.is_empty() {
        // read from stdin
        let mut stdin = io::stdin().lock();

        cat(&mut stdin, &args, &mut line_count, &mut stdout, &mut stderr);
    } else {
        for filename in &args.files {
            let mut file = match std::fs::File::open(filename) {
                Ok(f) => f,
                Err(err) => {
                    retval = 1;
                    eprintln!("cat: {}: {}", filename, err);
                    continue;
                }
            };

            cat(&mut file, &args, &mut line_count, &mut stdout, &mut stderr);
        }
    }

    stdout.flush().unwrap();
    stderr.flush().unwrap();

    process::exit(retval);
}

/// Cat a file from argument or stdin
fn cat<F: Read, O: Write, E: Write>(
    file: &mut F,
    args: &Args,
    line_count: &mut usize,
    stdout: &mut O,
    stderr: &mut E,
) {
    let mut character_count = 0;
    let mut at_line_start = false;
    let mut in_buffer: [u8; 8 * 8192] = [0; 8 * 8192];
    let mut out_buffer: Vec<u8> = Vec::with_capacity(24 * 8192);
    loop {
        let n_read = match file.read(&mut in_buffer) {
            Ok(n) => n,
            Err(err) => {
                writeln!(stderr, "cat: {err}").ok();
                break;
            }
        };
        if n_read == 0 {
            break;
        }

        for &byte in in_buffer[0..n_read].iter() {
            // Squeeze blank lines: skip before any output (including line numbers)
            if byte == b'\n' && character_count == 0 && args.squeeze_blank && at_line_start {
                continue;
            }

            // If we're tracking line numbers, this is where we'll print them out
            if character_count == 0 && (args.number || (args.number_nonblank && byte != b'\n')) {
                write!(out_buffer, "{: >6}  ", line_count).unwrap();
            }

            match byte {
                0..=8 | 11..=31 => {
                    if args.show_nonprinting {
                        push_caret(&mut out_buffer, byte + 64);
                    } else {
                        out_buffer.write_all(&[byte]).unwrap();
                    }
                    character_count += 1;
                }
                9 => {
                    out_buffer.write_all(args.tab().as_bytes()).unwrap();
                    character_count += 1;
                }
                10 => {
                    let is_blank = character_count == 0;

                    if is_blank {
                        at_line_start = true;
                    } else {
                        at_line_start = false;
                        character_count = 0;
                    }

                    // increment line count (skip for blank lines when -b)
                    if !is_blank || !args.number_nonblank {
                        *line_count += 1;
                    }

                    if args.show_ends {
                        out_buffer.write_all(b"$\n").unwrap();
                    } else {
                        out_buffer.write_all(b"\n").unwrap();
                    }
                }
                32..=126 => {
                    out_buffer.write_all(&[byte]).unwrap();
                    character_count += 1;
                }
                127 => {
                    if args.show_nonprinting {
                        push_caret(&mut out_buffer, b'?');
                    } else {
                        out_buffer.write_all(&[byte]).unwrap();
                    }
                    character_count += 1;
                }
                128..=159 => {
                    if args.show_nonprinting {
                        out_buffer.write_all(b"M-^").unwrap();
                        out_buffer.write_all(&[byte - 64]).unwrap();
                    } else {
                        out_buffer.write_all(&[byte]).unwrap();
                    }
                    character_count += 1;
                }
                _ => {
                    if args.show_nonprinting {
                        out_buffer.write_all(b"M-").unwrap();
                        out_buffer.write_all(&[byte - 128]).unwrap();
                    } else {
                        out_buffer.write_all(&[byte]).unwrap();
                    }
                    character_count += 1;
                }
            };
        }
        stdout.write_all(&out_buffer).unwrap();
        out_buffer.clear();
    }
}

fn push_caret<T: Write>(stdout: &mut T, notation: u8) {
    stdout.write_all(b"^").unwrap();
    stdout.write_all(&[notation]).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn default_args() -> Args {
        Args {
            show_all: false,
            number_nonblank: false,
            e: false,
            show_ends: false,
            number: false,
            squeeze_blank: false,
            t: false,
            show_tabs: false,
            u: false,
            show_nonprinting: false,
            files: vec![],
            output: None,
        }
    }

    fn run_cat(input: &[u8], args: &Args) -> String {
        let mut cursor = Cursor::new(input);
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();
        let mut line_count: usize = 1;
        cat(&mut cursor, args, &mut line_count, &mut stdout, &mut stderr);
        String::from_utf8(stdout).unwrap()
    }

    #[test]
    fn test_basic() {
        let output = run_cat(b"hello\nworld\n", &default_args());
        assert_eq!(output, "hello\nworld\n");
    }

    #[test]
    fn test_number_lines() {
        let mut args = default_args();
        args.number = true;
        let output = run_cat(b"a\nb\nc\n", &args);
        assert_eq!(output, "     1  a\n     2  b\n     3  c\n");
    }

    #[test]
    fn test_number_nonblank() {
        let mut args = default_args();
        args.number_nonblank = true;
        let output = run_cat(b"a\n\nb\n", &args);
        assert_eq!(output, "     1  a\n\n     2  b\n");
    }

    #[test]
    fn test_show_ends() {
        let mut args = default_args();
        args.show_ends = true;
        let output = run_cat(b"hello\nworld\n", &args);
        assert_eq!(output, "hello$\nworld$\n");
    }

    #[test]
    fn test_show_tabs() {
        let mut args = default_args();
        args.show_tabs = true;
        let output = run_cat(b"a\tb\n", &args);
        assert_eq!(output, "a^Ib\n");
    }

    #[test]
    fn test_squeeze_blank() {
        let mut args = default_args();
        args.squeeze_blank = true;
        let output = run_cat(b"a\n\n\n\nb\n", &args);
        assert_eq!(output, "a\n\nb\n");
    }

    // Regression: squeeze_blank must not skip non-blank lines.
    // Bug #1: the squeeze check inside the character_count > 0 branch
    // could skip content lines when at_line_start was true.
    #[test]
    fn test_squeeze_blank_preserves_content() {
        let mut args = default_args();
        args.squeeze_blank = true;
        let output = run_cat(b"a\n\n\nb\n\n\nc\n", &args);
        assert_eq!(output, "a\n\nb\n\nc\n");
    }

    // Regression: with -ns, squeezed blank lines should not be counted.
    // Bug #2: line_count was incremented before the squeeze check,
    // so squeezed lines consumed line numbers.
    #[test]
    fn test_squeeze_blank_with_number() {
        let mut args = default_args();
        args.squeeze_blank = true;
        args.number = true;
        let output = run_cat(b"a\n\n\n\nb\n", &args);
        // GNU cat -ns output: lines 1(a), 2(blank), 3(b)
        // The squeezed blank lines should not consume line numbers.
        assert_eq!(output, "     1  a\n     2  \n     3  b\n");
    }

    // Another squeeze+number edge case: multiple groups of blanks
    #[test]
    fn test_squeeze_blank_with_number_multiple_groups() {
        let mut args = default_args();
        args.squeeze_blank = true;
        args.number = true;
        let output = run_cat(b"a\n\n\nb\n\n\nc\n", &args);
        assert_eq!(
            output,
            "     1  a\n     2  \n     3  b\n     4  \n     5  c\n"
        );
    }

    #[test]
    fn test_show_nonprinting() {
        let mut args = default_args();
        args.show_nonprinting = true;
        // 0x01 = ^A, 0x7f = ^?
        let output = run_cat(&[0x01, b'a', 0x7f, b'\n'], &args);
        assert_eq!(output, "^Aa^?\n");
    }
}
