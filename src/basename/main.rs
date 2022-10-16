use std::path::MAIN_SEPARATOR;

use clap::Parser;

/// A rust implementation of basename
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Treat every argument as a string
    #[arg(short)]
    a: bool,

    /// The suffix to strip from the basename
    #[arg(short)]
    suffix: Option<String>,

    /// The path(s) to return the directory of
    args: Vec<String>,
}

fn main() {
    // usage: basename string [suffix]
    //        basename [-a] [-s suffix] string [...]
    let args = Args::parse();
    let basenames = run(args);
    for basename in &basenames {
        println!("{}", basename);
    }
}

fn run(args: Args) -> Vec<String> {
    let mut basenames: Vec<String> = Vec::new();

    if !args.a && args.suffix.is_none() && args.args.len() == 2 {
        // Got a string and suffix
        let path = shellexpand::tilde(&args.args[0]);
        let suffix = &args.args[1];
        let basename = &path;

        // Get the basename minus the suffix
        let bms = basename.strip_suffix(&*suffix).unwrap_or(&path);

        basenames.push(bms.to_string());
    } else if (args.a || args.args.len() == 1) || args.suffix.is_some() {
        // treat all args as strings
        for arg in &args.args {
            let path = shellexpand::tilde(&arg);
            let basename = get_basename(&path);

            if args.suffix.is_some() {
                let suffix = args.suffix.as_ref().unwrap();
                let bms = basename.strip_suffix(&*suffix).unwrap_or(&basename);
                basenames.push(bms.to_string());
            } else {
                basenames.push(basename);
            }
        }
    }
    basenames
}

fn get_basename(path: &str) -> String {
    match path.rfind(MAIN_SEPARATOR) {
        Some(idx) => {
            if idx == 0 {
                // The last separator is the first character, making it the dir
                MAIN_SEPARATOR.to_string()
            } else if path.ends_with(MAIN_SEPARATOR) {
                // Trim the trailing separator and try again
                get_basename(&path[..idx])
            } else {
                let basename = &path[(idx + 1)..];
                basename.to_string()
            }
        }
        None => '.'.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basename() {
        // Assert that we got the stats we were expecting
        assert_eq!(get_basename("/"), "/");
        assert_eq!(get_basename("/usr/bin/perl"), "perl");
        assert_eq!(get_basename("/usr/bin/"), "bin");
    }

    #[test]
    fn test_basename_path() {
        // basename /etc/motd
        let mut paths = Vec::new();
        paths.push(String::from("/etc/motd"));

        let args = Args {
            a: false,
            args: paths,
            suffix: None,
        };

        let basenames = run(args);
        assert_eq!(basenames.len(), 1);
        assert_eq!(basenames[0], "motd");
    }

    #[test]
    fn test_basename_paths() {
        // basename /etc/motd /etc/issue
        let mut paths = Vec::new();
        paths.push(String::from("/etc/motd"));
        paths.push(String::from("/etc/issue"));

        let args = Args {
            a: true,
            args: paths,
            suffix: None,
        };

        let basenames = run(args);
        assert_eq!(basenames.len(), 2);
        assert_eq!(basenames[0], "motd");
        assert_eq!(basenames[1], "issue");
    }

    #[test]
    fn test_basename_positional_suffix() {
        // basename /etc/init.d .d
        let mut paths = Vec::new();
        paths.push(String::from("/etc/init.d"));

        let args = Args {
            a: false,
            args: paths,
            suffix: Some(String::from(".d")),
        };

        let basenames = run(args);
        assert_eq!(basenames.len(), 1);
        assert_eq!(basenames[0], "init");
    }

    #[test]
    fn test_basename_arg_suffix() {
        // basename -s .rc /etc/mail.rc /etc/locate.rc
        let mut paths = Vec::new();
        paths.push(String::from("/etc/mail.rc"));
        paths.push(String::from("/etc/locate.rc"));

        let args = Args {
            a: false,
            args: paths,
            suffix: Some(String::from(".rc")),
        };

        let basenames = run(args);
        assert_eq!(basenames.len(), 2);
        assert_eq!(basenames[0], "mail");
        assert_eq!(basenames[1], "locate");
    }

    #[test]
    fn test_basename_arg_a_suffix() {
        // basename -a -s .rc /etc/mail.rc /etc/locate.rc
        let mut paths = Vec::new();
        paths.push(String::from("/etc/mail.rc"));
        paths.push(String::from("/etc/locate.rc"));

        let args = Args {
            a: true,
            args: paths,
            suffix: Some(String::from(".rc")),
        };

        let basenames = run(args);
        assert_eq!(basenames.len(), 2);
        assert_eq!(basenames[0], "mail");
        assert_eq!(basenames[1], "locate");
    }
}
