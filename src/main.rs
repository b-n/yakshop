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

mod shop;
mod yak;

use clap::Parser;
use shop::Shop;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::path::PathBuf;
use yak::Yak;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The location to the herd.xml file to use as a data source.
    #[arg(required = true)]
    pub herd: PathBuf,

    /// The number of days to run the simulation for.
    #[arg(required = true)]
    pub days: usize,
}

#[derive(Debug)]
pub enum YakShopError {
    ConfigFileNotFound(PathBuf),
}

impl Display for YakShopError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            YakShopError::ConfigFileNotFound(path) => {
                write!(f, "Config file {} not found", path.to_string_lossy())
            }
        }
    }
}

impl Error for YakShopError {}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let shop = Shop::try_from(&args.herd)?;

    println!("Day: {}\n\n{shop}", args.days);

    Ok(())
}
