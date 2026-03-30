use std::path::MAIN_SEPARATOR;

use clap::{Arg, ArgAction};
use tabled::{builder::Builder, settings::Style};

use coreutils::{clap_args, clap_base_command};

clap_args!(Args {
    flag zero: bool,
    multi paths: Vec<String>,
});

fn main() {
    let matches = clap_base_command()
        .arg(
            Arg::new("zero")
                .short('z')
                .long("zero")
                .action(ArgAction::SetTrue)
                .help("output a null-delimited list of dirnames (plain output only)"),
        )
        .arg(
            Arg::new("paths")
                .action(ArgAction::Append)
                .help("the path(s) to return the directory of")
                .required(true),
        )
        .get_matches();

    let args = Args::from_matches(&matches);

    let mut dirnames = Vec::new();
    for path in &args.paths {
        let dirname = get_dirname(path);
        dirnames.push(dirname);
    }

    if let Some(output) = &args.output {
        match output.as_str() {
            "table" => {
                let mut builder = Builder::new();
                builder.push_column(["Dirname(s)"]);

                for dirname in dirnames {
                    builder.push_record([dirname]);
                }
                let mut table = builder.build();
                println!("{}", table.with(Style::rounded()));
            }
            "json" => {
                println!("{}", serde_json::to_string(&dirnames).unwrap());
            }
            "yaml" => {
                println!("dirnames:");
                for dirname in dirnames {
                    println!("  - dirname: \"{}\"", dirname);
                }
            }
            _ => {
                for dirname in &dirnames {
                    if args.zero {
                        print!("{}\0", dirname);
                    } else {
                        println!("{}", dirname);
                    }
                }
            }
        }
    }
}

fn get_dirname(path: &str) -> String {
    // Strip trailing separators (but preserve root "/")
    let trimmed = path.trim_end_matches(MAIN_SEPARATOR);
    let trimmed = if trimmed.is_empty() {
        &path[..1]
    } else {
        trimmed
    };

    match trimmed.rfind(MAIN_SEPARATOR) {
        Some(0) => MAIN_SEPARATOR.to_string(),
        Some(idx) => trimmed[..idx].to_string(),
        None => '.'.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_root() {
        assert_eq!(get_dirname("/"), "/");
    }

    #[test]
    fn test_absolute_single_component() {
        assert_eq!(get_dirname("/foo"), "/");
    }

    #[test]
    fn test_bare_filename() {
        assert_eq!(get_dirname("foo"), ".");
    }

    #[test]
    fn test_relative_with_trailing_slash() {
        assert_eq!(get_dirname("foo/"), ".");
    }

    #[test]
    fn test_absolute_two_components() {
        assert_eq!(get_dirname("/home/stone"), "/home");
    }

    #[test]
    fn test_absolute_three_components() {
        assert_eq!(get_dirname("/home/stone/bin"), "/home/stone");
    }

    #[test]
    fn test_relative_nested() {
        assert_eq!(get_dirname("foo/bar"), "foo");
        assert_eq!(get_dirname("foo/bar/baz"), "foo/bar");
    }

    #[test]
    fn test_trailing_slashes_absolute() {
        assert_eq!(get_dirname("/home/stone/"), "/home");
        assert_eq!(get_dirname("/home/stone///"), "/home");
    }

    #[test]
    fn test_trailing_slashes_root() {
        assert_eq!(get_dirname("///"), "/");
    }

    #[test]
    fn test_absolute_single_component_trailing_slash() {
        assert_eq!(get_dirname("/foo/"), "/");
    }

    #[test]
    fn test_dot_and_dotdot() {
        assert_eq!(get_dirname("."), ".");
        assert_eq!(get_dirname(".."), ".");
    }
}
