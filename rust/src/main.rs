// This is free and unencumbered software released into the public domain.

use clientele::{
    StandardOptions,
    SysexitsError::{self, *},
    crates::clap::{Parser, Subcommand},
};
use edky::{PUBLIC_KEY_ENCODINGS, PublicKeyBytes, PublicKeyEncoding};

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
    /// List the supported public key encoding formats
    List {},

    /// Convert Ed25519 public keys between various encoding formats
    Convert {
        /// The input encoding
        #[clap(long, short, default_value = "hex")]
        from: PublicKeyEncoding,

        /// The output encoding
        #[clap(long, short, default_value = "hex")]
        to: PublicKeyEncoding,

        /// The input strings to convert
        inputs: Vec<String>,
    },

    /// Parse Ed25519 public keys in various encoding formats
    Parse {
        /// The input encoding
        #[clap(long, short, default_value = "hex")]
        from: PublicKeyEncoding,

        /// The input strings to convert
        inputs: Vec<String>,
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
        println!("edky {}", env!("CARGO_PKG_VERSION"));
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
        Command::List {} => {
            for (encoding, ..) in PUBLIC_KEY_ENCODINGS {
                println!("{}", encoding.to_string().to_lowercase());
            }
            Ok(())
        },
        Command::Convert { from, to, inputs } => {
            for input in inputs {
                let key = PublicKeyBytes::decode(from, input)?;
                let Some(encoded) = key.encode(to) else {
                    return Err(EX_CONFIG);
                };
                println!("{}", encoded);
            }
            Ok(())
        },
        Command::Parse { from, inputs } => {
            for input in inputs {
                let _ = PublicKeyBytes::decode(from, input)?;
            }
            println!("OK");
            Ok(())
        },
    }
}
