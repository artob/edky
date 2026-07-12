// This is free and unencumbered software released into the public domain.

//! Edky converts Ed25519 public keys between various encoding formats.

#![no_std]
#![allow(unused)]
#![cfg_attr(not(feature = "bytemuck"), forbid(unsafe_code))]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[doc = include_str!("../../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

mod parse_public_key_error;
pub use parse_public_key_error::*;

mod public_key_bytes;
pub use public_key_bytes::*;

mod public_key_encoding;
pub use public_key_encoding::*;

mod public_key_format;
pub use public_key_format::*;

mod public_key_formats;
pub use public_key_formats::*;
