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

// impl Args {
//     fn tab(&self) -> &'static str {
//         if self.show_tabs {
//             "^I"
//         } else {
//             "\t"
//         }
//     }
// }

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

    let mut blank: i32 = 0;

    if args.files.is_empty() {
        let mut ln: i32 = 0;

        // read from stdin
        let stdin = io::stdin();

        for line in stdin.lock().lines() {
            let buf = line.unwrap();

            if buf.trim().is_empty() {
                blank += 1;
            } else {
                blank = 0;
            }

            if (args.squeeze_blank && blank >= 1) || (!args.number_nonblank && args.number) {
                ln += 1;
            }

            // print to stdout
            // if !args.squeeze_blank || args.squeeze_blank && blank < 1 {
            if !(args.squeeze_blank && blank >= 1) {
                output(&args, &buf, ln);
            }
        }
    } else {
        for filename in &args.files {
            // TODO: match this
            let mut file = std::fs::File::open(filename).unwrap();

            let mut data = Vec::new();
            file.read_to_end(&mut data).expect("Unable to read data");

            // let mut out: String = String::new();
            let stdout = io::stdout();
            let mut writer = stdout.lock();

            // let mut out = Vec::new();

            // let mut pos = 0;
            for ch in data {
                // match ch {
                //     0..=8 | 10..=31 => writer.write_all(&[b'^', ch + 64]),
                //     // 7 => writer.write_all(&[b'^', ch + 64]),
                //     9 => writer.write_all(args.tab().as_bytes()),
                //     33..=126 => writer.write_all(&[b'M', b'-']),
                //     127 => writer.write_all(&[b'^', b'?']),
                //     128..=159 => writer.write_all(&[b'M', b'-', ch - 64]),
                //     160..=254 => writer.write_all(&[b'M', b'-', ch - 128]),
                //     _ => writer.write_all(&[b'M', b'-', b'^', b'?']),
                // }
                // .unwrap();

                match ch {
                    0..=8 => {
                        if args.show_nonprinting {
                            writer.write_all(&[ch + 64]).unwrap();
                        }
                    }
                    9 => {
                        if args.show_tabs {
                            writer.write_all(&[b'I']).unwrap();
                        } else {
                            writer.write_all(&[ch]).unwrap();
                        }
                    }
                    10 => {
                        if args.show_ends {
                            writer.write_all(b"$\n").unwrap();
                        } else {
                            writer.write_all(b"\n").unwrap();
                        }
                    }
                    32..=126 => {
                        writer.write_all(&[ch]).unwrap();
                    }
                    127 => {}
                    128..=159 => {
                        if args.show_nonprinting {
                            writer.write_all(b"M-^").unwrap();
                            writer.write_all(&[ch - 64]).unwrap();
                        } else {
                            writer.write_all(&[ch]).unwrap();
                        }
                    }
                    _ => {
                        if args.show_nonprinting {
                            writer.write_all(b"M-").unwrap();
                            writer.write_all(&[ch - 128]).unwrap();
                        } else {
                            writer.write_all(&[ch]).unwrap();
                        }
                    }
                };
            }

            writer.flush().unwrap();

            // let s = match str::from_utf8(&out) {
            //     Ok(v) => v,
            //     Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            // };

            // println!("{}", s);

            // // pretty sure I'm going to have to rewrite this whole section to read
            // // in possible binary files. As-is, this works for text but will fail
            // // hard if there's non-utf8 data, i.e., `cat -v /bin/bash`
            // let mut reader = io::BufReader::new(file);
            // let mut buf = String::new();
            // let mut ln: i32 = 0;

            // while reader.read_line(&mut buf).unwrap() > 0 {
            //     if buf.trim().is_empty() {
            //         blank += 1;
            //     } else {
            //         blank = 0;
            //     }

            //     // This isn't the prettiest, but it's functional
            //     if (args.squeeze_blank && blank >= 1)
            //         || (!args.number_nonblank && args.number)
            //         || (args.number_nonblank && blank == 0)
            //     {
            //         ln += 1;
            //     }

            //     // print to stdout
            //     if !(args.squeeze_blank && blank >= 1) {
            //         output(&args, &buf, ln);
            //     }

            //     // clear the buffer for the next read
            //     buf.clear();
            // }
        }
    }
}

/// Output the string according to stdargs
// going to have to change this to Vec<str> I think, so we can number lines
fn output(args: &Args, line: &str, number: i32) {
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

    // show line numbers
    if args.number && !args.number_nonblank {
        //     1	build:
        //     [...]
        //    10		cp target/release/dungeoncrawl dist
        // There's 3/4 spaces, right, justified, so we'll need some format! love
        // to get this right
        println!("{: >6} {}", number, s);
    } else if args.number_nonblank {
        if s.is_empty() {
            println!();
        } else {
            println!("{: >6} {}", number, s);
        }
    } else {
        println!("{}", s);
    }
}
