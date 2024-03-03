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
// Issue with the warp crate
#![allow(clippy::multiple_crate_versions)]

use clap::Parser;
use serde::Serialize;
use std::error::Error;
use std::net::IpAddr;
use std::path::PathBuf;
use warp::Filter;

use yakshop::{Products, Shop, Yak};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The location to the herd.xml file to use as a data source.
    #[arg(required = true)]
    pub herd: PathBuf,

    #[arg(long, default_value = "3000")]
    pub http_port: u16,

    #[arg(long, default_value = "127.0.0.1")]
    pub http_host: String,
}

#[derive(Serialize)]
struct HerdResponse {
    herd: Vec<Yak>,
}

#[derive(Serialize)]
struct StockResponse {
    milk: f64,
    wool: u32,
}

impl From<&Products> for StockResponse {
    fn from(products: &Products) -> Self {
        StockResponse {
            milk: products.milk(),
            wool: products.wool(),
        }
    }
}

/// Main entry point for the web server application.
///
/// The server accepts a single command line argument, the path to the herd.xml file to use as a data source.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let http_host: IpAddr = args.http_host.parse()?;
    let http_port = args.http_port;

    let shop = Shop::try_from(&args.herd)?;

    let home_page =
        warp::path::end().map(|| warp::reply::html(include_str!("../../static/index.html")));

    let stock = {
        let shop = shop.clone();
        warp::path!("yakshop" / "stock" / u32).map(move |days: u32| {
            let mut shop = shop.clone();
            shop.step_days(days);
            warp::reply::json(&StockResponse::from(&shop.produced_products))
        })
    };

    let herd = {
        let shop = shop.clone();
        warp::path!("yakshop" / "herd" / u32).map(move |days: u32| {
            let mut shop = shop.clone();
            shop.step_days(days);
            warp::reply::json(&HerdResponse {
                herd: shop.yaks().to_vec(),
            })
        })
    };

    let routes = warp::get().and(home_page.or(stock).or(herd));

    println!("Starting server on http://{http_host}:{http_port}");

    warp::serve(routes).run((http_host, http_port)).await;

    Ok(())
}
