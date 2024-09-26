/*
Luhny.rs by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the standard
/// "Result" enum.
use std::fmt::Result;

/// Importing the standard
/// "Display" trait.
use std::fmt::Display;

/// Importing the standard
/// "Error" trait.
use std::error::Error;

/// Importing the standard
/// "Formatter" trait.
use std::fmt::Formatter;

/// A data structure for
/// storing and handling errors.
#[derive(Clone,Eq,PartialEq, Debug)]
pub struct LuhnyErr {
    pub details: String
}

/// Implements functions
/// for the "LuhnyErr"
/// structure.
impl LuhnyErr {

    /// Implements a function to create
    /// a new instance of this data structure.
    pub fn new(details: &str) -> LuhnyErr {
        return LuhnyErr {
            details: details.to_owned()
        };
    }

    /// Implements a function to return
    /// a string representation of this 
    /// data structure.
    pub fn to_string(self) -> String {
        return self.details.to_string();
    }
}

/// Implements the error trait.
impl Error for LuhnyErr {
    fn description(&self) -> &str {
        &self.details
    }
}

/// Implements the Display trait
/// for the "LuhnyErr" structure.
impl Display for LuhnyErr {
    fn fmt(&self, f: &mut Formatter) -> Result {
        return write!(f,"{}",self.details);
    }
}