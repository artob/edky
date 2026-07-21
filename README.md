# Edky

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Package on Crates.io](https://img.shields.io/crates/v/edky)](https://crates.io/crates/edky)
[![Package on NPM](https://img.shields.io/npm/v/edky.js)](https://npmjs.com/package/edky.js)
[![Package on Pub.dev](https://img.shields.io/pub/v/edky)](https://pub.dev/packages/edky)
[![Package on PyPI](https://img.shields.io/pypi/v/edky)](https://pypi.org/project/edky)
[![Package on RubyGems](https://img.shields.io/gem/v/edky)](https://rubygems.org/gems/edky)

**Edky (pronounced *ed-key*) converts [Ed25519] public keys between various encoding formats.**

## ✨ Features

- Available both as the command-line tool [`edky`] and a polyglot library.
- Supports the Base16, Base32z, Base58, Base64, and Multibase encodings.
- Provides interop between ASIMOV, IPFS, Iroh, libp2p, NEAR, OpenSSH, etc.
- Polyglot software available for Dart, Python, Ruby, Rust, and TypeScript.
- Cuts red tape: 100% free and unencumbered public domain software.

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
<summary>Installation for Rust from Crates.io</summary>

#### Installation from [Crates.io]

```bash
cargo add edky
```
</details>

<details>
<summary>Installation for JavaScript/TypeScript from NPM</summary>

#### Installation from [NPM]

```bash
npm install edky.js
bun add edky.js
pnpm add edky.js
yarn add edky.js
```
</details>

<details>
<summary>Installation for Dart from Pub.dev</summary>

#### Installation from [Pub.dev]

```bash
dart pub add edky
flutter pub add edky
```
</details>

<details>
<summary>Installation for Python from PyPI</summary>

#### Installation from [PyPI]

```bash
pip install -U edky
uv add edky
poetry add edky
pdm add edky
```
</details>

<details>
<summary>Installation for Ruby from RubyGems</summary>

#### Installation from [RubyGems]

```bash
gem install edky
bundle add edky
```
</details>

## 👉 Examples

### Converting Ed25119 Public Keys via the CLI

```shellsession
$ edky convert -f iroh -t libp2p 47pjoycnsrfmxikm95jh13y88e8qnhzu5kungjpxyepgt7a8krpy
z6MktwupdmLXVVqTzCw4i46r4uGyosGXRnR3XjN4Zq7oMMsw

$ edky convert -f libp2p -t iroh z6MktwupdmLXVVqTzCw4i46r4uGyosGXRnR3XjN4Zq7oMMsw
47pjoycnsrfmxikm95jh13y88e8qnhzu5kungjpxyepgt7a8krpy

$ edky convert -f near -t hex ed25519:FVen3X669xLzsi6N2V91DoiyzHzg1uAgqiT8jZ9nS96Z
d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a

$ edky convert -f hex -t near d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a
ed25519:FVen3X669xLzsi6N2V91DoiyzHzg1uAgqiT8jZ9nS96Z
```

## 📚 Reference

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

[Crates.io]: https://crates.io/crates/edky
[NPM]: https://npmjs.com/package/edky.js
[Pub.dev]: https://pub.dev/packages/edky
[PyPI]: https://pypi.org/project/edky
[RubyGems]: https://rubygems.org/gems/edky

[Cargo]: https://rustup.rs
[Cargo Binstall]: https://crates.io/crates/cargo-binstall
[Ed25519]: https://en.wikipedia.org/wiki/Ed25519
