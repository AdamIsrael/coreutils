use std::path::MAIN_SEPARATOR;

use clap::{Arg, ArgAction};
use tabled::{builder::Builder, settings::Style};

use coreutils::{clap_args, clap_base_command};

clap_args!(Args {
    flag multiple: bool,
    maybe suffix: Option<String>,
    multi args: Vec<String>,

});

/// usage: basename string [suffix]
///        basename [-a] [-s suffix] string [...]
fn main() {
    let matches = clap_base_command()
        .arg(
            Arg::new("multiple")
                .short('a')
                .long("multiple")
                .action(ArgAction::SetTrue)
                .help("treat every argument as a string"),
        )
        .arg(
            Arg::new("suffix")
                .short('s')
                .help("the suffix to strip from the basename"),
        )
        .arg(
            Arg::new("args")
                .action(ArgAction::Append)
                .help("the path(s) to return the directory of"),
        )
        .get_matches();

    let args = Args::from_matches(&matches);

    let basenames = run(&args);

    if let Some(output) = &args.output {
        match output.as_str() {
            "table" => {
                let mut builder = Builder::new();
                builder.push_column(["Basename"]);

                for basename in basenames {
                    builder.push_record([basename]);
                }
                let mut table = builder.build();
                println!("{}", table.with(Style::rounded()));
            }
            "json" => {
                println!("{}", serde_json::to_string(&basenames).unwrap());
            }
            "yaml" => {
                println!("basenames:");
                for basename in basenames {
                    println!("  - basename: \"{}\"", basename);
                }
            }
            _ => {
                for basename in &basenames {
                    println!("{}", basename);
                }
            }
        }
    }
}

fn run(args: &Args) -> Vec<String> {
    let mut basenames: Vec<String> = Vec::new();

    if !args.multiple && args.suffix.is_none() && args.args.len() == 2 {
        // Got a string and suffix
        let path = &args.args[0];
        let suffix = &args.args[1];

        // Get the basename minus the suffix
        let basename = get_basename(path);
        let bms = basename.strip_suffix(suffix).unwrap_or(&basename);

        basenames.push(bms.to_string());
    } else if (args.multiple || args.args.len() == 1) || args.suffix.is_some() {
        // treat all args as strings
        for arg in &args.args {
            let basename = get_basename(arg);

            if let Some(suffix) = &args.suffix {
                let bms = basename.strip_suffix(suffix).unwrap_or(&basename);
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
        None => path.to_string(),
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
            multiple: false,
            args: paths,
            output: Some(String::from("plain")),
            suffix: None,
        };

        let basenames = run(&args);
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
            multiple: true,
            args: paths,
            output: Some(String::from("plain")),
            suffix: None,
        };

        let basenames = run(&args);
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
            multiple: false,
            args: paths,
            output: Some(String::from("plain")),
            suffix: Some(String::from(".d")),
        };

        let basenames = run(&args);
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
            multiple: false,
            args: paths,
            output: Some(String::from("plain")),
            suffix: Some(String::from(".rc")),
        };

        let basenames = run(&args);
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
            multiple: true,
            args: paths,
            output: Some(String::from("plain")),
            suffix: Some(String::from(".rc")),
        };

        let basenames = run(&args);
        assert_eq!(basenames.len(), 2);
        assert_eq!(basenames[0], "mail");
        assert_eq!(basenames[1], "locate");
    }
}
