#[macro_use]
extern crate lazy_static;

pub mod address;
pub mod base58;
pub mod hash;
pub mod key;
pub mod vanity;
pub mod error;

pub type CryptoResult<T> = Result<T, Box<dyn std::error::Error>>;