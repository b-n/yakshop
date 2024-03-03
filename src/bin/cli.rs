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

use clap::Parser;
use std::error::Error;
use std::path::PathBuf;

use yakshop::Shop;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The location to the herd.xml file to use as a data source.
    #[arg(required = true)]
    pub herd: PathBuf,

    /// The number of days to run the simulation for (max: U32_MAX)
    #[arg(required = true)]
    pub days: u32,
}

/// Main entry point for the CLI application. This application accepts a path to a herd.xml file and
/// a number of days to run the simulation for. It will then output the state of the shop at the end
/// of the simulation.
fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut shop = Shop::try_from(&args.herd)?;

    shop.step_days(args.days);

    println!("Day: {}\n\n{shop}", shop.elapsed_days);

    Ok(())
}
