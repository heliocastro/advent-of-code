extern crate regex;

use std::collections::HashMap;
use regex::Regex;

// Validate Birth Year
fn byr(register:String) -> bool {
    return (1920..=2002).contains(&register.parse::<u32>().unwrap())
}

// Validate Issue Year
fn iyr(register:String) -> bool {
    return (2010..=2020).contains(&register.parse::<u32>().unwrap())
}

// Validate Expiration Year
fn eyr(register:String) -> bool {
    return (2020..=2030).contains(&register.parse::<u32>().unwrap())
}

fn hgt(register:String) -> bool {
    let ( height, height_type ) = register.split_at(register.len() -2);
    match height_type {
        "cm" => return (150..=193).contains(&height.parse::<u32>().unwrap()),
        "in" => (59..=76).contains(&height.parse::<u32>().unwrap()),
        _ => return false,
    }
}

fn hcl(register:String) -> bool {
    let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    return re.is_match(&register);
}

fn ecl(register:String) -> bool {
    return vec![
        "amb".to_string(),
        "blu".to_string(),
        "brn".to_string(),
        "gry".to_string(),
        "grn".to_string(),
        "hzl".to_string(),
        "oth".to_string()
        ].contains(&register);
}

fn pid(register:String) -> bool {
    return register.len() == 9;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byr_test() {
        assert_eq!(byr("2002".to_string()), true);
        assert_eq!(byr("2003".to_string()), false);
    }

    #[test]
    fn hgt_test() {
        assert_eq!(hgt("60in".to_string()), true);
        assert_eq!(hgt("190cm".to_string()), true);
        assert_eq!(hgt("190in".to_string()), false);
        assert_eq!(hgt("190".to_string()), false);
    }

    #[test]
    fn ecl_test() {
        assert_eq!(ecl("brn".to_string()), true);
        assert_eq!(ecl("wat".to_string()), false);
    }

    #[test]
    fn pid_test() {
        assert_eq!(pid("000000001".to_string()), true);
        assert_eq!(pid("0123456789".to_string()), false);
    }
}


// Main Validation function
pub fn validate(data: HashMap<String,String>, m_accept: Vec<String>) -> bool {

    // North Pole Passport
    let fields: Vec<String> = vec![
        "byr".to_string(), // Birth Year
        "iyr".to_string(), // Issue Year
        "eyr".to_string(), // Expiration Year
        "hgt".to_string(), // Heigh
        "hcl".to_string(), // Hair Color
        "ecl".to_string(), // Eye Color
        "pid".to_string(), // Passport ID
        "cid".to_string() // Country ID
    ];


    for field in fields {
        if ! data.contains_key(&field) 
            && ! m_accept.contains(&field.to_string()) {
            return false;
        }
    }

    // Check Birth Year
    if ! byr(data.get("byr").unwrap().clone()) {
        return false;
    }

    // Check Issue Year
    if ! iyr(data.get("iyr").unwrap().clone()) { return false }

    // Check Expiraton year
    if ! eyr(data.get("eyr").unwrap().clone()) { return false }

    // Check height
    if ! hgt(data.get("hgt").unwrap().clone()) { return false }

    // Chwek Hair Color
    if ! hcl(data.get("hcl").unwrap().clone()) { return false }
 
    // Chwek Eye Color
    if ! ecl(data.get("ecl").unwrap().clone()) { return false }
  
    // Chwek Eye Color
    if ! pid(data.get("pid").unwrap().clone()) { return false }

    return true;
}