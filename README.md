# Yakshop!

A small app for calculating yak outputs for a webshop

## Running

### CLI

```console
you@yakshop > ./yakshop_cli --help
A simple yak shop simulator

Usage: yakshop_cli <HERD>

Arguments:
  <HERD>  The location to the herd.xml file to use as a data source

Options:
  -h, --help     Print help
  -V, --version  Print version
```

Run development mode with a herd.xml file:

`cargo run --bin yakshop_cli -- ./examples/herd.xml`

## Developing

Run Yakshop directly with cargo:

`cargo run --bin yakshop_cli -- ./examples/herd.xml`

Alterantively, a development release can be built and run:

`cargo build --bin yakshop_cli && ./target/debug/yakshop_cli`

Tests can be run using the cargo test runner:

`cargo test`

## Building

Build a production release as follows:

`cargo build --release --bin yakshop_cli`

The binary will be output to `./target/release/yakshop_cli`
