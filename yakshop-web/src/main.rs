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
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
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
struct YakResponse {
    name: String,
    age: f64,
    age_last_shaved: f64,
}

impl From<Yak> for YakResponse {
    fn from(yak: Yak) -> Self {
        YakResponse {
            name: yak.name().to_string(),
            age: yak.year_age(),
            age_last_shaved: yak.year_age_last_shaved(),
        }
    }
}

#[derive(Serialize)]
struct HerdResponse {
    herd: Vec<YakResponse>,
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

#[derive(Deserialize)]
struct OrderRequest {
    // Dead at present since it is never referenced
    #[allow(dead_code)]
    customer: String,
    order: OrderItems,
}

#[derive(Deserialize, Serialize)]
struct OrderItems {
    #[serde(skip_serializing_if = "Option::is_none")]
    milk: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skins: Option<u32>,
}

impl From<(Option<f64>, Option<u32>)> for OrderItems {
    fn from((milk, skins): (Option<f64>, Option<u32>)) -> Self {
        OrderItems { milk, skins }
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

    let home_page = warp::path::end()
        .and(warp::get())
        .map(|| warp::reply::html(include_str!("../../static/index.html")));

    let stock = {
        let shop = shop.clone();
        warp::path!("yakshop" / "stock" / u32)
            .and(warp::get())
            .and_then(move |days: u32| {
                let shop = shop.clone();
                get_stock(days, shop)
            })
    };

    let herd = {
        let shop = shop.clone();
        warp::path!("yakshop" / "herd" / u32)
            .and(warp::get())
            .and_then(move |days: u32| {
                let shop = shop.clone();
                get_herd(days, shop)
            })
    };

    let order = {
        let shop = shop.clone();

        warp::path!("yakshop" / "order" / u32)
            .and(warp::post())
            .and(warp::body::json())
            .and_then(move |days: u32, order: OrderRequest| {
                let shop = shop.clone();

                post_order(days, order, shop)
            })
    };

    println!("Starting server on http://{http_host}:{http_port}");

    let routes = home_page.or(stock).or(herd).or(order);
    warp::serve(routes).run((http_host, http_port)).await;

    Ok(())
}

#[allow(clippy::unused_async)]
#[allow(clippy::missing_errors_doc)]
async fn get_stock(day: u32, mut shop: Shop) -> Result<impl warp::Reply, Infallible> {
    shop.step_days(day);

    Ok(warp::reply::json(&StockResponse::from(
        &shop.produced_products,
    )))
}

#[allow(clippy::unused_async)]
#[allow(clippy::missing_errors_doc)]
async fn get_herd(day: u32, mut shop: Shop) -> Result<impl warp::Reply, Infallible> {
    shop.step_days(day);

    let yaks = shop.yaks().iter().map(|yak| YakResponse::from(yak.clone()));
    Ok(warp::reply::json(&HerdResponse {
        herd: yaks.collect(),
    }))
}

#[allow(clippy::unused_async)]
#[allow(clippy::missing_errors_doc)]
async fn post_order(
    day: u32,
    order: OrderRequest,
    mut shop: Shop,
) -> Result<impl warp::Reply, Infallible> {
    shop.step_days(day);

    // Consume the possible products from the shop
    let consumed_products = shop.consume_products(order.order.milk, order.order.skins);
    let response = OrderItems::from(consumed_products);

    // After consuming the order:
    // - If we can deliver everything, 201,
    // - If we can delivery only some goods, 206,
    // - If we can't deliver anything, 404
    let result = match consumed_products {
        (Some(_), Some(_)) => warp::reply::with_status(
            warp::reply::json(&response),
            warp::http::StatusCode::CREATED,
        ),
        (None, Some(_)) | (Some(_), None) => warp::reply::with_status(
            warp::reply::json(&response),
            warp::http::StatusCode::PARTIAL_CONTENT,
        ),
        (None, None) => {
            warp::reply::with_status(warp::reply::json(&()), warp::http::StatusCode::NOT_FOUND)
        }
    };

    Ok(result)
}
