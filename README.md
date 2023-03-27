# jsonlint

A simple `cli` utility to check the syntax of [JSON](https://www.json.org) files and/or pretty print them.

It's written in Rust, and uses the [json](https://crates.io/crates/json) library.

## USAGE
    jsonlint [OPTIONS] <file>

### ARGS
    <file>    JSON file to parse

### OPTIONS
    -h, --help       Print help information
    -p, --pretty     Pretty prints a JSON file
    -V, --version    Print version information

## Build

    make build
