extern crate clap;
extern crate advent_lib;

use clap::{Arg,App};
use std::collections::HashMap;

// Project internal moduels
use advent_lib::utils;

// Local modules
mod passport;

fn main() {
    let input_arguments = App::new("Advent 2020 Day One")
        .version("0.1.0")
        .author("Helio Chissini de Castro")
        .arg(Arg::with_name("input")
            .short("i")
            .takes_value(true)
            .required(true))
        .get_matches();

    // Load the input file
    let filename = input_arguments.value_of("input").unwrap();
    let batch = utils::read_input(filename);

    println!("{:?}", process_program(batch));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base() {
        let doc = utils::read_input("../unittest/day4-test-input.txt");
        assert_eq!(process_program(doc.clone()), 2);
    }

    #[test]
    fn invalid_batch() {
        let doc = utils::read_input("../unittest/day4-test-invalid.txt");
        assert_eq!(process_program(doc.clone()), 0);        
    }

    #[test]
    fn valid_batch() {
        let doc = utils::read_input("../unittest/day4-test-valid.txt");
        assert_eq!(process_program(doc.clone()), 4);        
    }
}

fn process_program(batch:Vec<String>) -> u32 {

    let mut registers:HashMap<String,String> = HashMap::new();
    let mut valid:u32 = 0;

    // This is the codes accepted to be missed
    let missing_acceptance_code:Vec<String> = vec![ "cid".to_string() ];
    
    for entry in batch {
        if entry.is_empty() {
            if passport::validation::validate(registers, missing_acceptance_code.clone()) {
                valid = valid + 1;
            }
            registers = HashMap::new();
        } else {
            let register: Vec<Vec<&str>> = entry.split(' ')
                .map(|x: &str| x.split(':').collect())
                .collect();
            for data in register{
                if data.len() == 2 {
                }
                    registers.insert(data[0].to_string(), data[1].to_string());
            }
        }
    }
    if ! registers.is_empty() {
        if passport::validation::validate(registers, missing_acceptance_code.clone()) {
            valid = valid + 1;
        }        
    }
    return valid;
}