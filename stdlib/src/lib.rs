use clap::{Command, arg, command, crate_version};

/// Returns the base clap command with common arguments and flags.
pub fn clap_base_command() -> Command {
    command!()
        .arg(
            arg!(-o --output <FORMAT> "Format to output to (plain, table, json, yaml)")
                .default_value("plain"),
        )
        .version(crate_version!())
}
