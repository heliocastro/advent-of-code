extern crate clap;

use clap::{Arg,App};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

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
    let input = File::open(filename)
        .expect("File not found !");        
    let data = BufReader::new(input);


    let mut valid:u32 = 0;
    let mut old_valid:u32 = 0;

    // Read all entry data and process
    for line in data.lines() {
        let pwd_data:String = line.unwrap().parse().unwrap();
        let entry:Vec<&str> = pwd_data.split(' ').collect();
        let range:Vec<&str> = entry[0].split('-').collect();
        let letter= entry[1].chars().next().unwrap();
        
        if process_program( 
            false,
            range[0].parse::<u32>().unwrap(),
            range[1].parse::<u32>().unwrap(),
            letter,
            entry[2].to_string() ) {
                old_valid = old_valid + 1;
        }
        
        if process_program( 
                true,
                range[0].parse::<u32>().unwrap(),
                range[1].parse::<u32>().unwrap(),
                letter,
                entry[2].to_string() ) {
                    valid = valid + 1;
        }
    }
    
    println!("Valid old passwords: {:?}", old_valid); 
    println!("Valid passwords: {:?}", valid);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        assert_eq!(process_program(false, 1, 3, 'a', "abcde".to_string()), true);
    }
    #[test]
    fn test_02() {
        assert_eq!(process_program(false,1, 3, 'b', "cdefg".to_string()), false);
    }
    #[test]
    fn test_03() {
        assert_eq!(process_program(false,2, 9, 'c', "ccccccccc".to_string()), true);
    } 

    #[test]
    fn test_04() {
        assert_eq!(process_program(true, 1, 3, 'a', "abcde".to_string()), true);
    }
    #[test]
    fn test_05() {
        assert_eq!(process_program(true, 1, 3, 'b', "cdefg".to_string()), false);
    }
    #[test]
    fn test_06() {
        assert_eq!(process_program(true, 2, 9, 'c', "ccccccccc".to_string()), false);
    } 
}

fn process_program(validation:bool, lower:u32, higher:u32, c:char, pwd:String) -> bool {
    let occur: Vec<&str> = pwd.matches(c).collect();
    let n = occur.len() as u32;
    
    if ! validation {
        return (lower..=higher).contains(&n);
    }
    
    let a= pwd.chars().nth((lower -1) as usize).unwrap() == c;
    let b= pwd.chars().nth((higher -1) as usize).unwrap() == c;

    if a != b {
        return true;
    } else {
        return false;
    }

}