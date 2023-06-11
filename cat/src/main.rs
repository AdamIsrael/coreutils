use std::io;
use std::io::prelude::*;
use std::str;

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

impl Args {
    fn tab(&self) -> &'static str {
        if self.show_tabs {
            "I"
        } else {
            "\t"
        }
    }
}

fn main() {
    let mut args = Args::parse();

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

    // let mut blank: i32 = 0;
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
                Err(_) => {
                    println!("cat: {}: No such file or directory", filename);
                    continue;
                }
            };

            cat(&mut file, &args, &mut line_count, &mut stdout, &mut stderr);
        }
    }

    stdout.flush().unwrap();
    stderr.flush().unwrap();
}

/// Cat a file from argument or stdin
fn cat<F: Read>(
    file: &mut F,
    args: &Args,
    line_count: &mut usize,
    stdout: &mut std::io::StdoutLock,
    stderr: &mut std::io::StderrLock,
) {
    let mut character_count = 0;
    let mut last_line_was_blank = false;
    let mut in_buffer: [u8; 8 * 8192] = [0; 8 * 8192];
    let mut out_buffer: Vec<u8> = Vec::with_capacity(24 * 8192);
    loop {
        let n_read = file.read(&mut in_buffer).unwrap();
        if n_read == 0 {
            break;
        }

        for &byte in in_buffer[0..n_read].iter() {
            // If we're tracking line numbers, this is where we'll print them out
            if character_count == 0 && (args.number || (args.number_nonblank && byte != b'\n')) {
                out_buffer
                    .write_all(format!("{: >6}  ", line_count).as_bytes())
                    .unwrap();

                last_line_was_blank = true;
            }

            match byte {
                0..=8 | 11..=31 => {
                    if args.show_nonprinting {
                        push_caret(&mut out_buffer, stderr, byte + 64);
                        count_character(&mut character_count, args);
                    }
                }
                9 => {
                    out_buffer.write_all(args.tab().as_bytes()).unwrap();
                    count_character(&mut character_count, args);
                }
                10 => {
                    // increment the line count when we find a newline
                    if character_count > 0 || !args.number_nonblank {
                        *line_count += 1;
                    }

                    if character_count == 0 {
                        if args.squeeze_blank && last_line_was_blank {
                            continue;
                        } else if !last_line_was_blank {
                            last_line_was_blank = true;
                        }
                    } else {
                        last_line_was_blank = false;
                        character_count = 0;
                    }

                    if args.show_ends {
                        out_buffer.write_all(b"$\n").unwrap();
                    } else {
                        out_buffer.write_all(b"\n").unwrap();
                    }
                }
                32..=126 => {
                    out_buffer.write_all(&[byte]).unwrap();
                    count_character(&mut character_count, args);
                }
                127 => {
                    push_caret(&mut out_buffer, stderr, b'?');
                    count_character(&mut character_count, args);
                }
                128..=159 => {
                    if args.show_nonprinting {
                        out_buffer.write_all(b"M-^").unwrap();
                        out_buffer.write_all(&[byte - 64]).unwrap();
                    } else {
                        out_buffer.write_all(&[byte]).unwrap();
                    }
                    count_character(&mut character_count, args);
                }
                _ => {
                    if args.show_nonprinting {
                        out_buffer.write_all(b"M-").unwrap();
                        out_buffer.write_all(&[byte - 128]).unwrap();
                    } else {
                        out_buffer.write_all(&[byte]).unwrap();
                    }
                    count_character(&mut character_count, args);
                }
            };
        }
        stdout.write_all(&out_buffer).unwrap();
        out_buffer.clear();
    }
}

fn count_character(character_count: &mut usize, args: &Args) {
    if args.number || args.number_nonblank {
        *character_count += 1;
    }
}

fn push_caret<T: Write>(stdout: &mut T, _stderr: &mut std::io::StderrLock, notation: u8) {
    stdout.write_all(&[b'^']).unwrap();
    stdout.write_all(&[notation]).unwrap();
}
