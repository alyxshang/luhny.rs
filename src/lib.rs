/*
Luhny.rs by Alyx Shang.
Licensed under the FSL v1.
*/

/// Declaring the
/// "modules"
/// directory as a module.
pub mod modules;

/// Re-exporting the 
/// module containing
/// this crate's 
/// error-handling
/// structure.
pub use modules::err::*;

/// Re-exporting the 
/// module containing
/// this crate's 
/// CLI.
pub use modules::cli::*;

/// Re-exporting the 
/// module containing
/// this crate's 
/// main APIs.
pub use modules::luhny::*;