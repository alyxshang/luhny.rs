/*
Luhny.rs by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the "App"
/// structure from the "cliply"
/// crate to make a new CLI
/// application.
use cliply::App;

/// Importing the structure
/// to catch and handle errors.
use super::err::LuhnyErr;

/// Importing the function to validate
/// an IMEI number.
use super::luhny::validate_imei;

/// This function contains Luhny's CLI.
/// A string or an error is returned.
pub fn cli() -> Result<String,LuhnyErr> {
    let mut luhny: App = App::new(
        "Luhny",
        "Alyx Shang",
        "0.1.0"
    );
    luhny.add_arg("chk", "check the supplied IMEI number", &true);
    if luhny.version_is(){
        Ok(luhny.version_info())
    }
    else if luhny.help_is(){
        Ok(luhny.help_info())
    }
    else if luhny.arg_was_used("chk"){
        let imei: String = match luhny.get_arg_data("chk"){
            Ok(imei) => imei,
            Err(e) => return Err::<String,LuhnyErr>(LuhnyErr::new(&e.to_string()))
        };
        let validation: bool = match validate_imei(&imei){
            Ok(validation) => validation,
            Err(e) => return Err::<String,LuhnyErr>(LuhnyErr::new(&e.to_string()))
        };

        if validation {
            let msg: String = format!("The IMEI number \"{}\" is valid.", imei);
            Ok(msg)
        }
        else {
            let msg: String = format!("The IMEI number \"{}\" is not valid.", imei);
            Ok(msg)
        }
    }
    else {
        Err::<String,LuhnyErr>(LuhnyErr::new(&luhny.help_info()))
    }
}