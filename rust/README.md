# Edky

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-2024%2B-blue)](https://endoflife.date/rust)
[![Package on Crates.io](https://img.shields.io/crates/v/edky)](https://crates.io/crates/edky)
[![Documentation](https://img.shields.io/docsrs/edky?label=docs.rs)](https://docs.rs/edky)

**Edky (pronounced *ed-key*) converts [Ed25519] public keys between various encoding formats.**

<sub>

[[Features](#-features)] |
[[Prerequisites](#%EF%B8%8F-prerequisites)] |
[[Installation](#%EF%B8%8F-installation)] |
[[Examples](#-examples)] |
[[Reference](#-reference)] |
[[Development](#%E2%80%8D-development)]

</sub>

<br/>

## ✨ Features

- Available both as the command-line tool [`edky`] and a Rust library.
- Supports the Base16, Base32z, Base58, Base64, and Multibase encodings.
- Provides interop between ASIMOV, IPFS, Iroh, libp2p, NEAR, OpenSSH, etc.
- 100% pure and safe Rust with minimal dependencies and no bloat.
- Designed for `#![no_std]` environment compatibility from the get-go.
- Supports opting out of any feature using comprehensive [feature flags].
- Adheres to the Rust API Guidelines in its [naming conventions].
- Polyglot software also available for Dart, Python, Ruby, and TypeScript.
- Cuts red tape: 100% free and unencumbered public domain software.

## 🛠️ Prerequisites

- [Rust] 2024+

## ⬇️ Installation

### Installation of the CLI

#### Installation via [Cargo Binstall]

```bash
cargo binstall -y edky
```

<img width="100%" alt="Installation via cargo-binstall" src="https://github.com/artob/edky/raw/master/rust/etc/asciinema/install.gif"/>

#### Installation via [Cargo]

```bash
cargo install edky --locked --features=cli
```

### Installation of the Library

<details>
<summary>Installation from Crates.io</summary>

#### Installation from [Crates.io]

```bash
cargo add edky
```
</details>

<details>
<summary>Configuration in `Cargo.toml`</summary>

#### Configuration in `Cargo.toml`

Enable all default features:

```toml
[dependencies]
edky = { version = "0" }
```

Enable only specific features:

```toml
[dependencies]
edky = { version = "0", default-features = false, features = ["alloc"] }
```
</details>

## 👉 Examples

### Converting Ed25119 Public Keys via the CLI

```bash
edky convert -f near -t openssh ed25519:FVen3X669xLzsi6N2V91DoiyzHzg1uAgqiT8jZ9nS96Z
```

### Importing the Library

```rust
use edky::{PublicKeyBytes, PublicKeyEncoding};
```

### Decoding Ed25519 Public Keys

```rust,no_run
# fn main() -> Result<(), Box<dyn core::error::Error>> {
use edky::{decode, PublicKeyEncoding::*};

let key = decode(Asimov, "ⒶYFVen3X669xLzsi6N2V91DoiyzHzg1uAgqiT8jZ9nS96Z")?;
let key = decode(Base16, "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a")?;
let key = decode(Base32z, "47pjoycnsrfmxikm95jh13y88e8qnhzu5kungjpxyepgt7a8krpy")?;
let key = decode(Base58, "FVen3X669xLzsi6N2V91DoiyzHzg1uAgqiT8jZ9nS96Z")?;
let key = decode(Base64, "11qYAYKxCrfVS/7TyWQHOg7hcvPapiMlrwIaaPcHURo=")?;
let key = decode(Base64Url, "11qYAYKxCrfVS/7TyWQHOg7hcvPapiMlrwIaaPcHURo")?;
let key = decode(Multibase, "z6MktwupdmLXVVqTzCw4i46r4uGyosGXRnR3XjN4Zq7oMMsw")?;
let key = decode(Near, "ed25519:FVen3X669xLzsi6N2V91DoiyzHzg1uAgqiT8jZ9nS96Z")?;
let key = decode(OpenSsh, "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAINdamAGCsQq31Uv+08lkBzoO4XLz2qYjJa8CGmj3B1Ea")?;
# Ok(()) }
```

### Encoding Ed25519 Public Keys

```rust
# fn main() -> Result<(), Box<dyn core::error::Error>> {
use edky::{PublicKeyBytes, PublicKeyEncoding::*};

let key = PublicKeyBytes::decode(Base16, "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a")?;

println!("{:?}", key.encode(Multibase));
# Ok(()) }
```

## 📚 Reference

[docs.rs/edky](https://docs.rs/edky)

### Command-Line Interface

```shellsession
$ edky
Edky converts Ed25519 public keys between various encoding formats

Usage: edky [OPTIONS] [COMMAND]

Commands:
  list     List the supported public key encoding formats
  convert  Convert Ed25519 public keys between various encoding formats
  parse    Parse Ed25519 public keys in various encoding formats
  help     Print this message or the help of the given subcommand(s)

Options:
      --color <COLOR>  Set the color output mode [default: auto] [possible values: auto, always, never]
  -d, --debug          Enable debugging output
      --license        Show license information
  -v, --verbose...     Enable verbose output (may be repeated for more verbosity)
  -V, --version        Print version information
  -h, --help           Print help (see more with '--help')
```

#### `edky list`

```shellsession
$ edky list --help
List the supported public key encoding formats

Usage: edky list [OPTIONS]

Options:
      --color <COLOR>  Set the color output mode [default: auto] [possible values: auto, always, never]
  -d, --debug          Enable debugging output
  -v, --verbose...     Enable verbose output (may be repeated for more verbosity)
  -h, --help           Print help
```

#### `edky convert`

```shellsession
$ edky convert --help
Convert Ed25519 public keys between various encoding formats

Usage: edky convert [OPTIONS] [INPUTS]...

Arguments:
  [INPUTS]...  The input strings to convert

Options:
      --color <COLOR>  Set the color output mode [default: auto] [possible values: auto, always, never]
  -f, --from <FROM>    The input encoding format [default: hex]
  -d, --debug          Enable debugging output
  -t, --to <TO>        The output encoding format [default: hex]
  -v, --verbose...     Enable verbose output (may be repeated for more verbosity)
  -h, --help           Print help
```

#### `edky parse`

```shellsession
$ edky parse --help
Parse Ed25519 public keys in various encoding formats

Usage: edky parse [OPTIONS] [INPUTS]...

Arguments:
  [INPUTS]...  The input strings to parse

Options:
      --color <COLOR>  Set the color output mode [default: auto] [possible values: auto, always, never]
  -f, --from <FROM>    The input encoding format [default: hex]
  -d, --debug          Enable debugging output
  -v, --verbose...     Enable verbose output (may be repeated for more verbosity)
  -h, --help           Print help
```

#### Encodings

| For `-f`, `-t`   | Sample Public Key |
| ---------------- | ----------------- |
| `asimov`         | ⒶYFVen3X669xLzsi6N2V91DoiyzHzg1uAgqiT8jZ9nS96Z |
| `base16`         | d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a |
| `base32z`        | 47pjoycnsrfmxikm95jh13y88e8qnhzu5kungjpxyepgt7a8krpy |
| `base58`         | FVen3X669xLzsi6N2V91DoiyzHzg1uAgqiT8jZ9nS96Z |
| `base64`         | 11qYAYKxCrfVS/7TyWQHOg7hcvPapiMlrwIaaPcHURo= |
| `base64url`      | 11qYAYKxCrfVS_7TyWQHOg7hcvPapiMlrwIaaPcHURo |
| `hex`            | d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a |
| `ipfs`           | z6MktwupdmLXVVqTzCw4i46r4uGyosGXRnR3XjN4Zq7oMMsw |
| `iroh`           | 47pjoycnsrfmxikm95jh13y88e8qnhzu5kungjpxyepgt7a8krpy |
| `libp2p`         | z6MktwupdmLXVVqTzCw4i46r4uGyosGXRnR3XjN4Zq7oMMsw |
| `multibase`      | z6MktwupdmLXVVqTzCw4i46r4uGyosGXRnR3XjN4Zq7oMMsw |
| `near`           | ed25519:FVen3X669xLzsi6N2V91DoiyzHzg1uAgqiT8jZ9nS96Z |
| `openssh`        | ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAINdamAGCsQq31Uv+08lkBzoO4XLz2qYjJa8CGmj3B1Ea |

### Feature Flags

#### Interoperability

| Feature          | Version  | Summary |
| ---------------- | -------- | ------- |
| `bytemuck`       | 1.25     | Implements `bytemuck::{Pod, Zeroable}` |
| `ed25519-dalek`  | 3.0      | Implements `From<ed25519_dalek::VerifyingKey>` |
| `eloquent`       | 2.1      | Implements `eloquent::ToSql` |
| `iroh1`          | 1.0      | Implements `From<iroh::{PublicKey, EndpointAddr}>` |
| `libsql`         | 0.9      | Implements `libsql::params::IntoValue` |
| `rocket`         | 0.5      | Implements `rocket::request::FromParam` |
| `serde`          | 1.0      | Derives `serde::{Serialize, Deserialize}` |
| `turso`          | 0.6      | Implements `turso::IntoValue` |
| `zerocopy`       | 0.8      | Derives `zerocopy::{FromBytes, IntoBytes}` |

## 👨‍💻 Development

```bash
git clone https://github.com/artob/edky.git
```

---

[![Share on X](https://img.shields.io/badge/share%20on-x-03A9F4?logo=x)](https://x.com/intent/post?url=https%3A%2F%2Fgithub.com%2Fartob%2Fedky&text=Edky)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https%3A%2F%2Fgithub.com%2Fartob%2Fedky&title=Edky)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hn-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https%3A%2F%2Fgithub.com%2Fartob%2Fedky&t=Edky)
[![Share on Facebook](https://img.shields.io/badge/share%20on-fb-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https%3A%2F%2Fgithub.com%2Fartob%2Fedky)
[![Share on LinkedIn](https://img.shields.io/badge/share%20on-linkedin-3949AB?logo=linkedin)](https://www.linkedin.com/sharing/share-offsite/?url=https%3A%2F%2Fgithub.com%2Fartob%2Fedky)

[`edky`]: https://github.com/artob/edky#command-line-interface

[feature flags]: https://docs.rs/crate/edky/latest/features
[naming conventions]: https://rust-lang.github.io/api-guidelines/naming.html

[Cargo]: https://rustup.rs
[Cargo Binstall]: https://crates.io/crates/cargo-binstall
[Ed25519]: https://en.wikipedia.org/wiki/Ed25519
[Rust]: https://rust-lang.org
