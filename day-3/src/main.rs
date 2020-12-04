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

    let mut slopes:Vec<String> = Vec::new();
    for line in data.lines() {
        let entry:String = line.unwrap().parse().unwrap();
        slopes.push(entry);
    }

    let test_slopes = vec![(1,1), (3,1), (5,1), (7,1), (1,2)];
    let trees = process_program(slopes.clone(), (3,1));
    let mut tree_probability:u64 = 0;

    for mtrees in test_slopes {
        if tree_probability == 0 {
            tree_probability = process_program(slopes.clone(), mtrees) as u64;
        } else {
            tree_probability = tree_probability * process_program(slopes.clone(), mtrees) as u64;
        }
    }

    println!("Number of trees for original 3,1 path: {}", trees);
    println!("Tree probability slope: {}", tree_probability);
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
        assert_eq!(process_program(path.clone(), (1,1)), 2);
        assert_eq!(process_program(path.clone(), (3,1)), 7);
        assert_eq!(process_program(path.clone(), (5,1)), 3);
        assert_eq!(process_program(path.clone(), (7,1)), 4);
        assert_eq!(process_program(path.clone(), (1,2)), 2);
    }
}

// Return number of trees
fn process_program(slopes:Vec<String>, walk:(u32, u32)) -> u32 {  
    let mut manipulated_path = slopes.clone();
    // First entry should be discarded as we will jump
    // down anyway
    let mut entry:String = manipulated_path.remove(0);
    let mut steps:u32 = 0;
    let mut trees:u32 = 0;

    while ! manipulated_path.is_empty() {
        for _down in 0..walk.1 {
            if manipulated_path.is_empty() {
                break;
            }
            entry = manipulated_path.remove(0);
        }      
        
        steps = steps + walk.0;

        // Duplicate as needed
        let dup = entry.clone();
        while steps >= entry.len() as u32 {
            entry.push_str(&dup.clone());
        }
        
        let cur:char = entry.chars().nth(steps as usize).unwrap();
        if cur == '#' {
            trees = trees + 1;
        }
    }
    return trees;
}