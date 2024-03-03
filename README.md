# Yakshop!

A small app for calculating yak outputs for a webshop

## Running

This repository contains code to run Yakshop in two different ways:

- CLI: A simple CLI for parsing XML and outputting the results of a simulation
- Web: a simple web server which can be used for generating JSON responses of
  the simulation

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

### Web server

```console
you@yakshop > ./yakshop_web --help
A simple yak shop simulator

Usage: yakshop_web [OPTIONS] <HERD>

Arguments:
  <HERD>  The location to the herd.xml file to use as a data source

Options:
      --http-port <HTTP_PORT>  [default: 3000]
      --http-host <HTTP_HOST>  [default: 127.0.0.1]
  -h, --help                   Print help
  -V, --version                Print version
```

Run development mode with a herd.xml file:

`cargo run --bin yakshop_web --features="web" -- ./examples/herd.xml`

## Developing

Run Yakshop directly with cargo:

`cargo run --bin yakshop_cli -- ./examples/herd.xml`

Alterantively, a development release can be built and run:

`cargo build --bin yakshop_cli && ./target/debug/yakshop_cli`

Tests can be run using the cargo test runner:

`cargo test`

## Building

The binaries are independant of each other so they need to be compiled
separately.

### CLI

`cargo build --release --bin yakshop_cli`

The binary will be output to `./target/release/yakshop_cli`

Run the final binary as follows:

```console
you@yakshop > ./target/release/yakshop_cli ./examples/herd.xml 13        
Day: 13

In Stock:
    1104.480 liters of milk
    3 skins of wool
Herd:
    Betty-1 4.13 years old
    Betty-2 8.13 years old
    Betty-3 9.63 years old
```

### Web Server

`cargo build --release --bin yakshop_cli --features="web"`

The binary will be output to `./target/release/yakshop_web`

Run the final binary as follows:

```console
you@yakshop > ./target/release/yakshop_web ./examples/herd.xml
Starting server on http://127.0.0.1:3000
```
