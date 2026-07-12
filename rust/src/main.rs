// This is free and unencumbered software released into the public domain.

use clientele::{
    StandardOptions,
    SysexitsError::{self, *},
    crates::clap::{Parser, Subcommand},
};
use edky::{PublicKeyBytes, PublicKeyEncoding};

/// Edky command-line interface (CLI)
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
    /// Convert Ed25519 public keys between various encoding formats
    Convert {
        /// The input encoding
        #[clap(long, short, default_value = "hex")]
        from: PublicKeyEncoding,

        /// The output encoding
        #[clap(long, short, default_value = "hex")]
        to: PublicKeyEncoding,

        /// The input string to convert
        input: String,
    },
}

pub fn main() -> Result<(), SysexitsError> {
    // Load environment variables from `.env`:
    clientele::dotenv().ok();

    // Expand wildcards and @argfiles:
    let args = clientele::args_os()?;

    // Parse command-line options:
    let options = Options::parse_from(args);

    // Print the program version, if requested:
    if options.flags.version {
        println!("Edky {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    // Print the program license, if requested:
    if options.flags.license {
        print!("{}", include_str!("../UNLICENSE"));
        return Ok(());
    }

    // Configure debug output:
    if options.flags.debug {}

    match options.command.unwrap() {
        Command::Convert { from: _, to, input } => {
            let key: PublicKeyBytes = input.parse()?;
            let Some(encoded) = key.encode(to) else {
                return Err(EX_CONFIG);
            };
            println!("{}", encoded);
            Ok(())
        },
    }
}
