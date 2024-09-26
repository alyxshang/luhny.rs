/*
Luhny.rs by Alyx Shang.
Licensed under the FSL v1.
*/

/// Exporting the 
/// module containing
/// this crate's 
/// error-handling
/// structure.
pub mod err;

/// Exporting the 
/// module containing
/// this crate's 
/// CLI.
pub mod cli;

/// Exporting the 
/// module containing
/// this crate's 
/// main APIs.
pub mod luhny;

/// Exporting the module
/// containing this crate's
/// tests.
#[cfg(test)]
pub mod tests;