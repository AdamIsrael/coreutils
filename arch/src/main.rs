use platform_info::*;
use serde_json::json;
use tabled::{builder::Builder, settings::Style};

use stdlib::{clap_args, clap_base_command};

clap_args!(Args {});

fn main() {
    let matches = clap_base_command().get_matches();
    let args = Args::from_matches(&matches);

    let arch = run();

    if let Some(output) = args.output {
        match output.as_str() {
            "table" => {
                let mut builder = Builder::new();
                builder.push_column(["Architecture"]);
                builder.push_record([arch]);
                let mut table = builder.build();
                println!("{}", table.with(Style::rounded()));
            }
            "json" => {
                let output = json!({
                    "architecture": arch,
                });

                println!("{}", serde_json::to_string(&output).unwrap());
            }
            "yaml" => println!("architecture: \"{arch}\""),
            _ => println!("{arch}"),
        }
    }
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
        // assert that we got _an_ architecture back
        assert_ne!(run().len(), 0);
    }
}
