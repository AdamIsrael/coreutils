use clap::Parser;
use platform_info::*;

/// Print machine architecture
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {}

fn main() {
    let arch = run();

    println!("{}", arch);
}

fn run() -> String {
    let uname = PlatformInfo::new().unwrap();
    uname.machine().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_architecture() {
        // assert that we got _a_ architecture back
        assert_ne!(run().len(), 0);
    }
}
