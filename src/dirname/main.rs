use std::path::MAIN_SEPARATOR;

use clap::Parser;
use shellexpand;

/// A rust implementation of dirname
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path to return the directory of
    paths: Vec<String>,
}

fn main() {
    let args = Args::parse();

    for arg in &args.paths {
        let path = shellexpand::tilde(&arg);

        let dirname = get_dirname(&path);
        println!("{}", dirname);
    }
}

fn get_dirname(path: &str) -> String {
    let dirname = match path.rfind(MAIN_SEPARATOR) {
        Some(idx) => {
            if idx == 0 {
                // The last separator is the first character, making it the dir
                MAIN_SEPARATOR.to_string()
            } else if path.starts_with(MAIN_SEPARATOR) == false {
                /*
                if the string doesn't start with the separator, i.e., "foo/"
                then the dirname is always '.'
                */
                '.'.to_string()
            } else {
                let dirname = &path[..idx];
                dirname.to_string()
            }
        }
        None => '.'.to_string(),
    };
    dirname
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dirname() {
        // Assert that we got the stats we were expecting
        assert_eq!(get_dirname("/"), "/");
        assert_eq!(get_dirname("/foo"), "/");
        assert_eq!(get_dirname("foo"), ".");
        assert_eq!(get_dirname("foo/"), ".");
        assert_eq!(get_dirname("/home/stone"), "/home");
        assert_eq!(get_dirname("/home/stone/bin"), "/home/stone");
    }
}
