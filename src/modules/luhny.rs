/*
Luhny.rs by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the function
/// to check whether a string
/// is an integer or not from
/// the "coutils" crate.
use coutils::is_int;

/// Importing the structure
/// to catch and handle errors.
use super::err::LuhnyErr;

/// This function checks whether a string consists of
/// only digits. If this is the case, `true` is returned.
/// If this is not the case, `false` is returned.
pub fn is_number_sequence(subject: &String) -> bool {
    let mut result: bool = true;
    let subject_characters: Vec<char> = subject.chars().collect();
    for item in subject_characters{
        if is_int(&item.to_string()){}
        else {
            result = false;
        }
    }
    result
}

/// This function iterates through
/// all items in a vector, doubles each,
/// and adds them to a new vector. If the
/// operation fails, an error is returned.
pub fn double_numbers_in_arr(
    num_array: &Vec<usize>
) -> Result<Vec<usize>,LuhnyErr> {
    if num_array.is_empty(){
        let e: String = format!("The supplied vector is empty.");
        Err::<Vec<usize>,LuhnyErr>(LuhnyErr::new(&e.to_string()))
    }
    else {
        let mut result: Vec<usize> = Vec::new();
        for num in num_array {
            result.push(num*2);
        }
        Ok(result)
    }
}

/// This function attempts to get every
/// item whose index is divisible by 2.
/// If this operation fails, an error
/// is returned.
pub fn get_important_items<T: Clone>(
    subject: &Vec<T>
) -> Result<Vec<T>, LuhnyErr> {
    if subject.is_empty(){
        let e: String = format!("The supplied vector is empty.");
        Err::<Vec<T>,LuhnyErr>(LuhnyErr::new(&e.to_string()))
    }
    else {
        let mut result: Vec<T> = Vec::new();
        for (idx,item) in subject.iter().enumerate(){
            let actual_idx: usize = idx + 1;
            if actual_idx % 2 == 0 {
                result.push(item.clone());
            }
            else {}
        }
        Ok(result)
    }
}

/// This function attempts to get every
/// item whose index is not divisible by 2.
/// If this operation fails, an error
/// is returned.
pub fn get_other_items<T: Clone>(
    subject: &Vec<T>
) -> Result<Vec<T>, LuhnyErr> {
    if subject.is_empty(){
        let e: String = format!("The supplied vector is empty.");
        Err::<Vec<T>,LuhnyErr>(LuhnyErr::new(&e.to_string()))
    }
    else {
        let mut result: Vec<T> = Vec::new();
        for (idx,item) in subject.iter().enumerate(){
            let actual_idx: usize = idx + 1;
            if actual_idx % 2 != 0 {
                result.push(item.clone());
            }
            else {}
        }
        Ok(result)
    }
}

/// This function attempts to
/// add all the numbers from a number
/// array in a lump sum. If this operation
/// fails, an error is returned.
pub fn add_array_numbers(
    num_array: &Vec<usize>, 
) -> Result<usize,LuhnyErr> {
    if num_array.is_empty(){
        let e: String = format!("The supplied vector is empty.");
        Err::<usize,LuhnyErr>(LuhnyErr::new(&e.to_string()))
    }
    else {
        let mut res: usize = 0;
        for num in num_array {
            res += num;
        }
        Ok(res)
    }
}

/// This functions attempts to convert a 
/// string to a vector of numbers. These 
/// numbers are of type "usize".
/// If this operation fails, an error is
/// returned.
pub fn convert_string_to_num_array(
    subject: &String
) -> Result<Vec<usize>, LuhnyErr> {
    if subject.is_empty(){
        let e: String = "The supplied string is empty.".to_string();
        Err::<Vec<usize>,LuhnyErr>(LuhnyErr::new(&e.to_string()))
    }
    else {
        if is_number_sequence(subject){
            let e: String = "The supplied string is not a number sequence.".to_string();
            Err::<Vec<usize>,LuhnyErr>(LuhnyErr::new(&e.to_string()))
        }      
        else {
            let mut res: Vec<usize> = Vec::new();
            let chars: Vec<char> = subject.chars().collect();
            for character in chars {
                let sub: String = character.to_string();
                let integer: usize = match sub.parse::<usize>(){
                    Ok(integer) => integer,
                    Err(e) => return Err::<Vec<usize>,LuhnyErr>(LuhnyErr::new(&e.to_string()))
                };
                res.push(integer);
            }
            Ok(res)
        }  
    }
}

/// This functions attempts to validate an IMEI
/// number. This number has to be 15 characters long.
/// If the IMEI number is valid, "true" is returned. If it
/// is not valid, "false" is returned. If the validation operation
/// fails, an error is returned.
pub fn validate_imei(imei: &String) -> Result<bool,LuhnyErr> {
    if imei.is_empty(){
        let e: String = "The supplied string is empty.".to_string();
        Err::<bool,LuhnyErr>(LuhnyErr::new(&e.to_string()))
    }
    else {
        let mut result: bool = false;
        let imei_chars: Vec<char> = imei.chars().collect();
        let last_char: String = (imei_chars[imei_chars.len()-1]).to_string();
        let int_arr: Vec<usize> = match convert_string_to_num_array(imei){
            Ok(int_arr) => int_arr,
            Err(e) => return Err::<bool,LuhnyErr>(LuhnyErr::new(&e.to_string()))
        };
        let other_items: Vec<usize> = match get_other_items(&int_arr){
            Ok(other_items) => other_items,
            Err(e) => return Err::<bool,LuhnyErr>(LuhnyErr::new(&e.to_string()))
        };
        let important_items: Vec<usize> = match get_important_items(&int_arr){
            Ok(important_items) => important_items,
            Err(e) => return Err::<bool,LuhnyErr>(LuhnyErr::new(&e.to_string()))
        };
        let important_doubles: Vec<usize> = match double_numbers_in_arr(&important_items){
            Ok(important_doubles) => important_doubles,
            Err(e) => return Err::<bool,LuhnyErr>(LuhnyErr::new(&e.to_string()))
        };
        let other_sum: usize = match add_array_numbers(&other_items){
            Ok(other_sum) => other_sum,
            Err(e) => return Err::<bool,LuhnyErr>(LuhnyErr::new(&e.to_string()))
        };
        let important_double_sum = match add_array_numbers(&important_doubles){
            Ok(important_double_sum) => important_double_sum,
            Err(e) => return Err::<bool,LuhnyErr>(LuhnyErr::new(&e.to_string()))
        };
        let sum: usize = other_sum + important_double_sum;
        let computed_cd: String = (10 - (sum%10)).to_string();
        if last_char == computed_cd && imei.len() == 15 {
            result = true;
        }
        else {}
        Ok(result)
    }
}