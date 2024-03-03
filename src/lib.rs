// Setup Clippy lints
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![allow(unknown_lints)]
#![warn(missing_debug_implementation)]
#![warn(missing_copy_implementation)]
#![warn(rust_2018_idioms)]
#![warn(rust_2021_compatibility)]
#![warn(trivial_casts, trivial_numeric_casts)]
#![warn(unused_qualifications)]
#![warn(variant_size_difference)]

#[cfg(test)]
#[macro_use]
extern crate approx;

mod shop;
mod yak;

use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::path::PathBuf;

pub use shop::Shop;
pub use yak::{Products, Yak};

#[derive(Debug)]
pub enum YakShopError {
    ConfigFileNotFound(PathBuf),
    ConfigFileParseError(String),
}

impl Display for YakShopError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            YakShopError::ConfigFileNotFound(path) => {
                write!(f, "Config file {} not found", path.to_string_lossy())
            }
            YakShopError::ConfigFileParseError(msg) => {
                write!(f, "Error parsing config file: {msg}")
            }
        }
    }
}

impl Error for YakShopError {}
