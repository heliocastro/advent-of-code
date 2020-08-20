extern crate clap;

use clap::{Arg,App};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let input_arguments = App::new("Advent Day Two")
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
    
    for line in data.lines() {
        let mut orig_prog: Vec<usize> = line.unwrap().split(',').map(|x| x.parse::<usize>().unwrap()).collect(); 
        for noun in 0..99 {
            for verb in 0..99 {
                let mut prog:Vec<usize> = orig_prog.to_vec();
                // Adjust input
                prog[1] = noun;
                prog[2] = verb;
                prog = process_program(prog);
                if prog[0] == 19690720 {
                    println!("Resulting verb noum: {:?} {:?}", prog[1], prog[2]);
                    println!("Result: {:?}", 100 * prog[1] + prog[2]);
                    return;
                }
                println!("{:?} {:?} = {:?}", prog[1], prog[2], prog[0]);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_02() {
        assert_eq!(process_program([1,0,0,0,99].to_vec()), [2,0,0,0,99]);
        assert_eq!(process_program([2,3,0,3,99].to_vec()), [2,3,0,6,99]);
        assert_eq!(process_program([2,4,4,5,99,0].to_vec()), [2,4,4,5,99,9801]);
        assert_eq!(process_program([1,1,1,4,99,5,6,0,99].to_vec()), [30,1,1,4,2,5,6,0,99]);
    } 
}

fn process_program(mut prog:Vec<usize>) -> Vec<usize> {
    
    for data in (0..(prog).len()).step_by(4) {
        if prog[data] == 99 {
            break;
        }
        let pos = prog[data+3]; 
        match prog[data] {
            1 => prog[pos] = prog[prog[data+1]] + prog[prog[data+2]],
            2 => prog[pos] = prog[prog[data+1]] * prog[prog[data+2]],
            _ => println!("Error !"),
        }
    }

    return prog;
}