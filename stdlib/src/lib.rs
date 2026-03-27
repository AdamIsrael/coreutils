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

#[macro_export]
macro_rules! clap_args {
    // Entry point
    ($name:ident { $($body:tt)* }) => {
        $crate::clap_args!(@parse $name { fields[] rest[$($body)*] });
    };

    // Terminal rule: all fields consumed — automatically includes base command fields (output)
    (@parse $name:ident { fields[ $({ $kind:ident $field:ident : $ty:ty [$($default:tt)*] })* ] rest[] }) => {
        struct $name {
            output: Option<String>,
            $( $field: $ty ),*
        }

        impl $name {
            fn from_matches(matches: &::clap::ArgMatches) -> Self {
                Self {
                    output: matches.get_one::<String>("output").cloned(),
                    $(
                        $field: $crate::clap_args!(@extract $kind matches $field [$($default)*]),
                    )*
                }
            }
        }
    };

    // Parsing rules: consume one field at a time into accumulator

    (@parse $name:ident { fields[ $($acc:tt)* ] rest[ flag $field:ident : $ty:ty, $($rest:tt)* ] }) => {
        $crate::clap_args!(@parse $name { fields[ $($acc)* { flag $field : $ty [] } ] rest[ $($rest)* ] });
    };

    (@parse $name:ident { fields[ $($acc:tt)* ] rest[ opt $field:ident : $ty:ty, $($rest:tt)* ] }) => {
        $crate::clap_args!(@parse $name { fields[ $($acc)* { opt $field : $ty [] } ] rest[ $($rest)* ] });
    };

    (@parse $name:ident { fields[ $($acc:tt)* ] rest[ maybe $field:ident : $ty:ty, $($rest:tt)* ] }) => {
        $crate::clap_args!(@parse $name { fields[ $($acc)* { maybe $field : $ty [] } ] rest[ $($rest)* ] });
    };

    (@parse $name:ident { fields[ $($acc:tt)* ] rest[ multi $field:ident : $ty:ty, $($rest:tt)* ] }) => {
        $crate::clap_args!(@parse $name { fields[ $($acc)* { multi $field : $ty [] } ] rest[ $($rest)* ] });
    };

    (@parse $name:ident { fields[ $($acc:tt)* ] rest[ value($default:expr) $field:ident : $ty:ty, $($rest:tt)* ] }) => {
        $crate::clap_args!(@parse $name { fields[ $($acc)* { value $field : $ty [$default] } ] rest[ $($rest)* ] });
    };

    // Extraction rules

    (@extract flag $matches:ident $field:ident []) => {{
        $matches.get_flag(stringify!($field))
    }};

    (@extract opt $matches:ident $field:ident []) => {{
        $matches.get_one::<String>(stringify!($field)).cloned().unwrap_or_default()
    }};

    (@extract maybe $matches:ident $field:ident []) => {{
        $matches.get_one::<String>(stringify!($field)).cloned()
    }};

    (@extract multi $matches:ident $field:ident []) => {{
        $matches.get_many::<String>(stringify!($field))
            .unwrap_or_default()
            .cloned()
            .collect()
    }};

    (@extract value $matches:ident $field:ident [$default:expr]) => {{
        $matches.get_one::<String>(stringify!($field))
            .and_then(|v| v.parse().ok())
            .unwrap_or($default)
    }};
}
