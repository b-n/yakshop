# Yakshop-web

Run the yakshop Web Server!

## Running

```console
you@yakshop > ./yakshop-web --help
A Web Server for the Yak Shop

Usage: yakshop-web [OPTIONS] <HERD>

Arguments:
  <HERD>  The location to the herd.xml file to use as a data source

Options:
      --http-port <HTTP_PORT>  [default: 3000]
      --http-host <HTTP_HOST>  [default: 127.0.0.1]
  -h, --help                   Print help
  -V, --version                Print version
```

Run development mode with a herd.xml file:

```console
you@yakshop:yakshop-cli > cargo run -- ../examples/herd.xml`
Starting server on http://127.0.0.1:3000
```

## Developing

Run Yakshop directly with cargo:

`cargo run -- ../examples/herd.xml`

Alterantively, a development release can be built and run:

`cargo build && ../target/debug/yakshop-web` (if building from `yakshop-cli/`)

## Building

`cargo build --release`

The binary will be output to `../target/release/yakshop-web`

Run the final binary as follows:

```console
you@yakshop:yakshop-web > ../target/release/yakshop-web ../examples/herd.xml  
Starting server on http://127.0.0.1:3000
```

## Building and running with Docker

The build needs to be triggered with a context of the root of this project.

Assuming you are running the build from this directory, then:

```console
docker build -t yakshop-web -f Dockerfile ..
```

And run the container:

```console
docker run -v "$PWD/../:/app" -p 3000:3000 yakshop-web --http-host 0.0.0.0 "/app/examples/herd.xml" 
```
