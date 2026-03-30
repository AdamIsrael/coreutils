use clap::{Arg, ArgAction};

use coreutils::{clap_args, clap_base_command};

clap_args!(Args {
    flag no_newline: bool,
    flag escape: bool,
    multi args: Vec<String>,
});

/// Echo the arguments
fn main() {
    let matches = clap_base_command()
        .arg(
            Arg::new("no_newline")
                .action(ArgAction::SetTrue)
                .help("Do not print the trailing newline character")
                .short('n'),
        )
        .arg(
            Arg::new("escape")
                .action(ArgAction::SetTrue)
                .help("Enable interpretation of backslash escapes")
                .short('e'),
        )
        .arg(
            Arg::new("args")
                .action(ArgAction::Append)
                .help("Arguments to echo to stdout"),
        )
        .mut_args(|a| {
            // Hide the base --output argument, since it doesn't make sense for `echo`.
            if a.get_id() == "output" {
                a.hide(true)
            } else {
                a
            }
        })
        .get_matches();
    let args = Args::from_matches(&matches);

    let output = args.args.join(" ");

    if args.escape {
        print!("{}", interpret_escapes(&output));
    } else {
        print!("{output}");
    }
    if !args.no_newline {
        println!();
    }
}

fn interpret_escapes(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('\\') => result.push('\\'),
                Some('a') => result.push('\x07'),
                Some('b') => result.push('\x08'),
                Some('c') => break,
                Some('f') => result.push('\x0C'),
                Some('n') => result.push('\n'),
                Some('r') => result.push('\r'),
                Some('t') => result.push('\t'),
                Some('v') => result.push('\x0B'),
                Some('0') => {
                    let mut val: u32 = 0;
                    for _ in 0..3 {
                        let mut peek = chars.clone();
                        if let Some(d) = peek.next() {
                            if ('0'..='7').contains(&d) {
                                val = val * 8 + d.to_digit(8).unwrap();
                                chars.next();
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    result.push(char::from_u32(val).unwrap_or('\0'));
                }
                Some(other) => {
                    result.push('\\');
                    result.push(other);
                }
                None => result.push('\\'),
            }
        } else {
            result.push(c);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpret_escapes_newline() {
        assert_eq!(interpret_escapes("hello\\nworld"), "hello\nworld");
    }

    #[test]
    fn test_interpret_escapes_tab() {
        assert_eq!(interpret_escapes("hello\\tworld"), "hello\tworld");
    }

    #[test]
    fn test_interpret_escapes_backslash() {
        assert_eq!(interpret_escapes("hello\\\\world"), "hello\\world");
    }

    #[test]
    fn test_interpret_escapes_stop() {
        assert_eq!(interpret_escapes("hello\\cworld"), "hello");
    }

    #[test]
    fn test_interpret_escapes_octal() {
        assert_eq!(interpret_escapes("\\0101"), "A"); // octal 101 = 65 = 'A'
    }

    #[test]
    fn test_interpret_escapes_bell() {
        assert_eq!(interpret_escapes("\\a"), "\x07");
    }

    #[test]
    fn test_interpret_escapes_no_escape() {
        assert_eq!(interpret_escapes("hello world"), "hello world");
    }

    #[test]
    fn test_interpret_escapes_unknown() {
        assert_eq!(interpret_escapes("\\z"), "\\z");
    }
}
