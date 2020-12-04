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

    let mut path:Vec<String> = Vec::new();
    for line in data.lines() {
        let entry:String = line.unwrap().parse().unwrap();
        path.push(entry);
    }

    let trees = process_program(path);

    println!("Number of trees: {}", trees);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let path:Vec<String> = vec![
            "..##.......".to_string(),
            "#...#...#..".to_string(),
            ".#....#..#.".to_string(),
            "..#.#...#.#".to_string(),
            ".#...##..#.".to_string(),
            "..#.##.....".to_string(),
            ".#.#.#....#".to_string(),
            ".#........#".to_string(),
            "#.##...#...".to_string(),
            "#...##....#".to_string(),
            ".#..#...#.#".to_string(),
        ]; 
        assert_eq!(process_program(path), 7)
    }
}

// Return number of trees
fn process_program(path:Vec<String>) -> u32 {
    
    let mut manipulated_path = path.clone();
    // First entry should be discarded as we will jump
    // down anyway
    let mut entry:String = manipulated_path.remove(0);
    let path_lenght:u32 = entry.len() as u32;
    let mut steps:u32 = 0;
    let mut trees:u32 = 0;

    for _distance in 0..path.len() {
        if manipulated_path.is_empty() {
            manipulated_path = path.clone();
        }
        entry = manipulated_path.remove(0);
        steps = steps + 3;
        if path_lenght <= steps {
            steps = steps - path_lenght;
        }
        
        let cur:char = entry.chars().nth(steps as usize).unwrap();
        if cur == '#' {
            trees = trees + 1;
        }
    }
    return trees;
}