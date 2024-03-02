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
use serde::Deserialize;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::path::PathBuf;

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

#[derive(Deserialize, Debug)]
enum YakSex {
    #[serde(rename = "f")]
    Female,
    #[serde(rename = "m")]
    Male,
}

#[derive(Deserialize, Debug)]
#[allow(never_read)]
struct Yak {
    name: String,
    age: f32,
}

impl Display for Yak {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} years old", self.name, self.age,)
    }
}

#[derive(Default, Deserialize, Debug)]
pub struct Shop {
    #[serde(rename = "$value")]
    yaks: Vec<Yak>,
    #[serde(skip_deserializing)]
    milk: f64,
    #[serde(skip_deserializing)]
    skins: usize,
}

impl Display for Shop {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"In Stock:
    {} liters of milk
    {} skins of wool
Herd:"#,
            self.milk, self.skins
        )?;

        for yak in &self.yaks {
            write!(f, "\n    {yak}")?;
        }

        Ok(())
    }
}

impl TryFrom<&PathBuf> for Shop {
    type Error = YakShopError;

    fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
        let herd_config = path.as_path();
        if !herd_config.exists() {
            return Err(YakShopError::ConfigFileNotFound(path.clone()));
        }

        let herd_xml =
            std::fs::read_to_string(herd_config).expect("Could not read herd.xml file to string");

        let shop: Shop = serde_xml_rs::from_str(&herd_xml).expect("Could not parse herd.xml file");

        Ok(shop)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let shop = Shop::try_from(&args.herd)?;

    println!("Day: {}\n\n{shop}", args.days);

    Ok(())
}
