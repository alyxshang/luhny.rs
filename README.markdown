# LUHNY.RS :iphone: :lock: :crab:

![GitHub CI](https://github.com/alyxshang/luhny.rs/actions/workflows/rust.yml/badge.svg)

***A library and tool to validate unique device identifiers. :iphone: :lock: :crab:***

## ABOUT :books:

This repository contains the source code for the Rust crate, ***Luhny.rs***. This crate serves as both a library and CLI tool allowing users to validate IMEI numbers with a length of 15 characters.

## INSTALLATION :inbox_tray:

### Installation as a library

To use ***Luhny.rs*** as a library in your own Rust project, add the following line to the `dependencies` section of your Rust project's `Cargo.toml` file:

```TOML
luhny = { git = "https://github.com/alyxshang/luhny.rs", tag = "v.0.1.0" }
```

### Installation as a CLI tool

To install ***Luhny.rs*** as a CLI tool on your own system, run the following command from a terminal session:

```bash
cargo install --git https://github.com/alyxshang/luhny.rs --tag v.0.1.0
```

To run the command above, you will have to have the Rust toolchain installed.

## USAGE :hammer:

### API Documentation

To view the documentation of the APIs this crate provides, run the command `cargo doc --open` from the root of this repository.

### CLI Tool

***Luhny.rs*** features the following three CLI commands: i) getting version information, ii) getting help information, and iii) validating an IMEI number.

- To print out version information about ***Luhny.rs***, you can run either of these three commands:

```bash
luhny -v
# OR
luhny version
# OR
luhny --version
```

- To print out help information about ***Luhny.rs***, you can run either of these three commands:

```bash
luhny -h
# OR
luhny help
# OR
luhny --help
```

- To validate an IMEI number, you can run either of these three commands (`IMEI_NUM` is representative for your IMEI number):

```bash
luhny -c IMEI_NUM
# OR
luhny chk IMEI_NUM
# OR
luhny --chk IMEI_NUM
```


## CHANGELOG :black_nib:

### Version 0.1.0

- Initial release.
- Upload to GitHub.

## NOTE :scroll:

- *Luhny.rs :iphone: :lock: :crab:* by *Alyx Shang :black_heart:*.
- Licensed under the [FSL v1](https://github.com/alyxshang/fair-software-license).