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

    // Read all entry data in a Vec
    let mut report_data:Vec<u32> = Vec::new();
    for line in data.lines() {
        report_data.push(line.unwrap().parse::<u32>().unwrap());
    }
    let res = process_program(report_data);
    println!("Result data = {} {}", res.0, res.1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        assert_eq!(process_program(vec![1721,979,366,299,675,1456]),
        (514579,241861950));
    } 
}

fn process_program(mut data:Vec<u32>) -> (u32,u32) {  
    data.sort();
    
    let mut two_solution:u32 = 0;
    let mut three_solution:u32 = 0;

    // Iteract with the    
    loop {
        let cur:u32 = data.pop().unwrap();
        for elem in data.iter() {
            if cur + elem == 2020 {
                two_solution = cur * elem;
            }
            for elem3 in data.iter() {
                if elem3 == elem { continue; }
                
                if cur + elem + elem3 == 2020 {
                    three_solution = cur * elem * elem3;
                }
            }

            if two_solution > 0 && three_solution > 0 {
                return (two_solution, three_solution);
            }
        }
    }
}
