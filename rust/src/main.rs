// This is free and unencumbered software released into the public domain.

//! Command-line argument and diagnostic policy.
//!
//! A subcommand is required except for help, version, and license requests.
//! `convert` and `parse` require at least one input; successful `parse` prints
//! `OK` only after every supplied input has been validated.
//!
//! Diagnostics go to stderr: `-v` adds a summary, `-vv` adds per-item progress,
//! and `-vvv` or `--debug` also shows the parsed command. `--color` controls help,
//! usage errors, and progress diagnostics. Key output on stdout is plain text.

use clientele::{
    StandardOptions,
    SysexitsError::{self, *},
    crates::clap::{
        ColorChoice, CommandFactory, FromArgMatches, Parser, Subcommand, error::ErrorKind,
    },
};
use core::str::FromStr;
use derive_more::FromStrError;
use edky::{PUBLIC_KEY_FORMATS, PublicKeyBytes, PublicKeyEncoding};
use std::{ffi::OsString, io::IsTerminal};

/// Edky converts Ed25519 public keys between various encoding formats.
#[derive(Debug, Parser)]
#[command(name = "Edky", long_about)]
#[command(arg_required_else_help = true)]
struct Options {
    #[clap(flatten)]
    flags: StandardOptions,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// List the supported public key encoding formats.
    List {},

    /// Convert Ed25519 public keys between various encoding formats.
    Convert {
        /// The input encoding format.
        #[clap(long, short, default_value = "hex", value_parser = parse_format)]
        from: PublicKeyEncoding,

        /// The output encoding format.
        #[clap(long, short, default_value = "hex", value_parser = parse_format)]
        to: PublicKeyEncoding,

        /// The input strings to convert.
        #[arg(required = true)]
        inputs: Vec<String>,
    },

    /// Parse Ed25519 public keys in various encoding formats.
    Parse {
        /// The input encoding format.
        #[clap(long, short, default_value = "hex", value_parser = parse_format)]
        from: PublicKeyEncoding,

        /// The input strings to parse.
        #[arg(required = true)]
        inputs: Vec<String>,
    },
}

/// Runs the CLI, returning errors from argument expansion or key processing.
///
/// Invalid command-line syntax exits with Clap's usage-error status (2).
pub fn main() -> Result<(), SysexitsError> {
    // Load environment variables from `.env`:
    clientele::dotenv().ok();

    // Expand wildcards and @argfiles:
    let args = clientele::args_os()?;

    // Parse command-line options:
    let mut cli = Options::command().color(color_choice(&args));
    let matches = cli
        .try_get_matches_from_mut(args)
        .unwrap_or_else(|error| error.exit());
    let options = Options::from_arg_matches(&matches).unwrap_or_else(|error| error.exit());

    // Print the program version, if requested:
    if options.flags.version {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    // Print the program license, if requested:
    if options.flags.license {
        print!("{}", include_str!("../UNLICENSE"));
        return Ok(());
    }

    let Some(command) = options.command else {
        cli.error(ErrorKind::MissingSubcommand, "a subcommand is required")
            .exit();
    };

    let diagnostics = Diagnostics::new(&options.flags);
    diagnostics.log(3, format_args!("debug: {command:?}"));

    match command {
        Command::List {} => {
            for format in PUBLIC_KEY_FORMATS {
                println!("{}", format.name());
                diagnostics.log(
                    2,
                    format_args!(
                        "{}: encoding={}, prefix={:?}",
                        format.name(),
                        format.encoding(),
                        format.prefix()
                    ),
                );
            }
            diagnostics.log(
                1,
                format_args!("listed {} formats", PUBLIC_KEY_FORMATS.len()),
            );
            Ok(())
        },
        Command::Convert { from, to, inputs } => {
            let count = inputs.len();
            for (index, input) in inputs.into_iter().enumerate() {
                let key = PublicKeyBytes::decode(from, input)?;
                let Some(encoded) = key.encode(to) else {
                    return Err(EX_CONFIG);
                };
                println!("{}", encoded);
                diagnostics.log(2, format_args!("converted input {}/{}", index + 1, count));
            }
            diagnostics.log(
                1,
                format_args!("converted {count} public key(s) from {from} to {to}"),
            );
            Ok(())
        },
        Command::Parse { from, inputs } => {
            let count = inputs.len();
            for (index, input) in inputs.into_iter().enumerate() {
                let _ = PublicKeyBytes::decode(from, input)?;
                diagnostics.log(2, format_args!("validated input {}/{}", index + 1, count));
            }
            println!("OK");
            diagnostics.log(1, format_args!("validated {count} public key(s) as {from}"));
            Ok(())
        },
    }
}

// Select colors before Clap can exit while displaying help or a usage error.
// Invalid values are left to Clap; arguments after `--` are always input data.
fn color_choice(args: &[OsString]) -> ColorChoice {
    let mut args = args
        .iter()
        .skip(1)
        .take_while(|arg| arg.as_os_str() != "--");
    let mut color = ColorChoice::Auto;
    while let Some(arg) = args.next() {
        let value = if arg == "--color" {
            args.next().and_then(|value| value.to_str())
        } else {
            arg.to_str()
                .and_then(|value| value.strip_prefix("--color="))
        };
        if let Some(value) = value.and_then(|value| value.parse().ok()) {
            color = value;
        }
    }
    color
}

struct Diagnostics {
    verbosity: u8,
    color: bool,
}

impl Diagnostics {
    fn new(flags: &StandardOptions) -> Self {
        Self {
            verbosity: if flags.debug { u8::MAX } else { flags.verbose },
            color: match flags.color {
                ColorChoice::Always => true,
                ColorChoice::Never => false,
                ColorChoice::Auto => {
                    std::io::stderr().is_terminal() && std::env::var_os("NO_COLOR").is_none()
                },
            },
        }
    }

    fn log(&self, level: u8, message: core::fmt::Arguments<'_>) {
        if self.verbosity >= level {
            if self.color {
                eprintln!("\x1b[36medky:\x1b[0m {message}");
            } else {
                eprintln!("edky: {message}");
            }
        }
    }
}

fn parse_format(input: &str) -> Result<PublicKeyEncoding, FromStrError> {
    // TODO: support an "auto" value that detects the format from the input
    for format in PUBLIC_KEY_FORMATS {
        if format.name() == input {
            return Ok(format.encoding());
        }
    }
    PublicKeyEncoding::from_str(input)
}
