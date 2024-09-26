/*
Luhny.rs by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the "validate_imei"
/// function to test it.
use super::luhny::validate_imei;

/// Importing the "get_other_items"
/// function to test it.
use super::luhny::get_other_items;

/// Importing the "add_array_numbers"
/// function to test it.
use super::luhny::add_array_numbers;

/// Importing the "is_number_sequence"
/// function to test it.
use super::luhny::is_number_sequence;

/// Importing the "get_important_items"
/// function to test it.
use super::luhny::get_important_items;

/// Importing the "double_numbers_in_arr"
/// function to test it.
use super::luhny::double_numbers_in_arr;

/// Importing the "convert_string_to_num_array"
/// function to test it.
use super::luhny::convert_string_to_num_array;

/// Testing the "validate_imei" function.
#[test]
pub fn test_validate_imei() -> () {
    match validate_imei(&"353879234252633".to_string()){
        Ok(res) => assert_eq!(res,true),
        Err(e) => eprintln!("{}", &e)
    };
    match validate_imei(&"353879234252634".to_string()){
        Ok(res) => assert_eq!(res,false),
        Err(e) => eprintln!("{}", &e)
    };
}

/// Testing the "get_other_items" function.
#[test]
pub fn test_get_other_items() -> () {
    let start: Vec<usize> = vec![1,2,3,4,5,6];
    let test_res: Vec<usize> = vec![1,3,5];
    match get_other_items(&start){
        Ok(res) => assert_eq!(res, test_res),
        Err(e) => eprintln!("{}", &e)
    };
}

/// Testing the "get_important_items" function.
#[test]
pub fn test_get_important_items() -> () {
    let start: Vec<usize> = vec![1,2,3,4,5,6];
    let test_res: Vec<usize> = vec![2,4,6];
    match get_important_items(&start){
        Ok(res) => assert_eq!(res, test_res),
        Err(e) => eprintln!("{}", &e)
    };
}

/// Testing the "add_array_numbers" function.
#[test]
pub fn test_add_array_numbers() -> () {
    let start: Vec<usize> = vec![1,2,3,4];
    let end: usize = 10;
    match add_array_numbers(&start){
        Ok(res) => assert_eq!(res, end),
        Err(e) => eprintln!("{}", &e)
    };
}

/// Testing the "is_number_sequence" function.
#[test]
pub fn test_is_number_sequence() -> () {
    let start_true: String = "12345".to_string();
    let start_false: String = "1234X".to_string();
    assert_eq!(is_number_sequence(&start_true),true);
    assert_eq!(is_number_sequence(&start_false),false);
}

/// Testing the "double_numbers_in_arr" function.
#[test]
pub fn test_double_numbers_in_arr() -> () {
    let start: Vec<usize> = vec![1,2,3];
    let end: Vec<usize> = vec![2,4,6];
    match double_numbers_in_arr(&start){
        Ok(res) => assert_eq!(res,end),
        Err(e) => eprintln!("{}", &e)
    };
}

/// Testing the "convert_string_to_num_array" function.
#[test]
pub fn test_convert_string_to_num_array() -> () {
    let start: String = "123".to_string();
    let end: Vec<usize> = vec![1,2,3];
    match convert_string_to_num_array(&start){
        Ok(res) => assert_eq!(res,end),
        Err(e) => eprintln!("{}", &e)
    };
}
