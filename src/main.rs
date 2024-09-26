/*
Luhny.rs by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing this crate's
/// CLI function.
use luhny::cli;

/// The main
/// point of entry
/// for the Rust compiler.
fn main(){
    match cli(){
        Ok(res) => println!("{}", &res),
        Err(e) => eprintln!("{}", &e)
    };
}