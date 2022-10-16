use std::path::MAIN_SEPARATOR;

use clap::Parser;

/// A rust implementation of basename
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path to return the directory of
    path: String,
}

fn main() {
    let args = Args::parse();

    let path = shellexpand::tilde(&args.path);

    let dirname = get_dirname(&path);
    println!("{}", dirname);
}

fn get_dirname(path: &str) -> String {
    let idx = path.rfind(MAIN_SEPARATOR).unwrap_or(0);

    if idx > 0 {
        let dirname = &path[..idx];
        dirname.to_string()
    } else {
        path.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dirname() {
        // Assert that we got the stats we were expecting
        assert_eq!(get_dirname("/"), "/");
        assert_eq!(get_dirname("/home/stone"), "/home");
        assert_eq!(get_dirname("/home/stone/bin"), "/home/stone");
    }
}
