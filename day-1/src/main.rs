extern crate clap;

use clap::{Arg,App};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let input_arguments = App::new("Advent Day One")
        .version("0.1.0")
        .author("Helio Chissini de Castro")
        .arg(Arg::with_name("input")
            .short("i")
            .takes_value(true)
            .required(true))
        .get_matches();

    let filename = input_arguments.value_of("input").unwrap();

    let input = File::open(filename)
        .expect("File not found !");
        
    let data = BufReader::new(input);
    
    let mut total:i32 = 0;

    for line in data.lines() {
        let mass:i32 = line.unwrap().parse().unwrap();
        total = total + get_fuel_data(mass);
    }
    println!(r#"Total Fuel Needed: {:?}"#, total);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_01() {
        assert_eq!(2, get_fuel_data(12));
        assert_eq!(2, get_fuel_data(14));
        assert_eq!(654, get_fuel_data(1969));
        assert_eq!(33583, get_fuel_data(100756));
    } 
}

fn get_fuel_data(mass: i32) -> i32 {
    return (mass/3)-2;
}