# Yakshop!

A small app for calculating yak outputs for a webshop

## Running

```console
you@yakshop > ./yakshop --help
A simple yak shop simulator

Usage: yakshop <HERD>

Arguments:
  <HERD>  The location to the herd.xml file to use as a data source

Options:
  -h, --help     Print help
  -V, --version  Print version
```

Run development mode with a herd.xml file:

`cargo run -- ./examples/herd.xml`

## Developing

Run Yakshop directly with cargo:

`cargo run -- ./examples/herd.xml`

Alterantively, a development release can be built and run:

`cargo build && ./target/debug/yakshop`

Tests can be run using the cargo test runner:

`cargo test`

## Building

Build a production release as follows:

`cargo build --release`

The binary will be output to `./target/release/yakshop`
