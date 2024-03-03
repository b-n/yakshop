# Yakshop-cli

Run the yakshop from your terminal!

## Running

```console
you@yakshop > ./yakshop-cli --help
A simple yak shop simulator

Usage: yakshop-cli <HERD>

Arguments:
  <HERD>  The location to the herd.xml file to use as a data source

Options:
  -h, --help     Print help
  -V, --version  Print version
```

Run development mode with a herd.xml file:

```console
you@yakshop:yakshop-cli > cargo run -- ../examples/herd.xml 13`
In Stock:
    1104.480 liters of milk
    3 skins of wool
Herd:
    Betty-1 4.13 years old
    Betty-2 8.13 years old
    Betty-3 9.63 years old
```

## Developing

Run Yakshop directly with cargo:

`cargo run -- ../examples/herd.xml 13`

Alterantively, a development release can be built and run:

`cargo build && ../target/debug/yakshop-cli` (if building from `yakshop-cli/`)

## Building

`cargo build --release`

The binary will be output to `../target/release/yakshop-cli`

Run the final binary as follows:

```console
you@yakshop:yakshop-cli > ../target/release/yakshop-cli ../examples/herd.xml 13        
In Stock:
    1104.480 liters of milk
    3 skins of wool
Herd:
    Betty-1 4.13 years old
    Betty-2 8.13 years old
    Betty-3 9.63 years old
```

## Building and running with Docker

The build needs to be triggered with a context of the root of this project.

Assuming you are running the build from this directory, then:

```console
docker build -t yakshop-cli -f Dockerfile ..
```

And to run the container:

```console
docker run -it --rm -v "$PWD/../:/app" yakshop-cli "/app/examples/herd.xml" 13
```
