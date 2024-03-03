# Yakshop!

A small app for calculating yak outputs for a webshop

## Running

This app has two frontends:

- yakshop-cli - a shell UI which runs a simulation
  ```console
  cargo run -p yakshop-cli -- ./examples/herd.xml 13
  ```
  Note: see [yakshop-cli/README.md](yakshop-cli/README.md) for more info
- yakshop-web - a web server which can be called to run simulations
  ```console
  cargo run -p yakshop-web -- ./examples/herd.xml
  ```
  Note: see [yakshop-web/README.md](yakshop-web/README.md) for more info


## Developing

Use one of the frontends to test the application directly.

To run tests:

```console
cargo test --workspace
```
