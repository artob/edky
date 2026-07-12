# Edky.rs

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.85%2B-blue)](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/)
[![Package](https://img.shields.io/crates/v/edky)](https://crates.io/crates/edky)
[![Documentation](https://img.shields.io/docsrs/edky?label=docs.rs)](https://docs.rs/edky)

**Edky converts Ed25519 public keys between various encoding formats.**

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

- Supports Ed25119 public keys encoded as Base16, Base58, Base64, and Z32.
- 100% pure and safe Rust with minimal dependencies and no bloat.
- Designed for `no_std` environment compatibility from the get-go.
- Supports opting out of any feature using comprehensive [feature flags].
- Adheres to the Rust API Guidelines in its [naming conventions].
- Cuts red tape: 100% free and unencumbered public domain software.
- Polyglot software also available for Dart, Python, Ruby, and TypeScript.

## 🛠️ Prerequisites

- [Rust] 1.85+ (2024 edition)

## ⬇️ Installation

### Installation via Cargo

```bash
cargo add edky
```

### Installation in `Cargo.toml`

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

## 👉 Examples

### Importing the Library

```rust
use edky::{PublicKeyBytes, PublicKeyEncoding};
```

## 📚 Reference

[docs.rs/edky](https://docs.rs/edky)

### Feature Flags

#### Encodings

| Feature         | Example Public Key |
| :-------------- | :----------------- |
| `asimov`        | ⒶYFVen3X669xLzsi6N2V91DoiyzHzg1uAgqiT8jZ9nS96Z
| `base16`        | d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a
| `base58`        | FVen3X669xLzsi6N2V91DoiyzHzg1uAgqiT8jZ9nS96Z
| `base64`        | 11qYAYKxCrfVS/7TyWQHOg7hcvPapiMlrwIaaPcHURo=
| `near`          | ed25519:FVen3X669xLzsi6N2V91DoiyzHzg1uAgqiT8jZ9nS96Z
| `openssh`       | ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAINdamAGCsQq31Uv+08lkBzoO4XLz2qYjJa8CGmj3B1Ea
| `z32`           | 47pjoycnsrfmxikm95jh13y88e8qnhzu5kungjpxyepgt7a8krpy

#### Interoperability

| Feature         | Version | Summary |
| :-------------- | :------ | :------ |
| `bytemuck`      | 1.25    | Implements `bytemuck::{Pod, Zeroable}`
| `ed25519-dalek` | 3.0     | Implements `From<ed25519_dalek::VerifyingKey>`
| `eloquent`      | 2.1     | Implements `eloquent::ToSql`
| `iroh`          | 1.0     | Implements `From<iroh::{PublicKey, EndpointAddr}>`
| `libsql`        | 0.9     | Implements `libsql::params::IntoValue`
| `rocket`        | 0.5     | Implements `rocket::request::FromParam`
| `serde`         | 1.0     | Derives `serde::{Serialize, Deserialize}`
| `turso`         | 0.6     | Implements `turso::IntoValue`
| `zerocopy`      | 0.8     | Derives `zerocopy::{FromBytes, IntoBytes}`

## 👨‍💻 Development

```bash
git clone https://github.com/artob/edky.git
```

---

[![Share on X](https://img.shields.io/badge/share%20on-x-03A9F4?logo=x)](https://x.com/intent/post?url=https://github.com/artob/edky&text=Edky)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https://github.com/artob/edky&title=Edky)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hn-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https://github.com/artob/edky&t=Edky)
[![Share on Facebook](https://img.shields.io/badge/share%20on-fb-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/artob/edky)
[![Share on LinkedIn](https://img.shields.io/badge/share%20on-linkedin-3949AB?logo=linkedin)](https://www.linkedin.com/sharing/share-offsite/?url=https://github.com/artob/edky)

[feature flags]: https://docs.rs/crate/edky/latest/features
[naming conventions]: https://rust-lang.github.io/api-guidelines/naming.html

[Rust]: https://rust-lang.org
