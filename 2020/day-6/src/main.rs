extern crate clap;
extern crate advent_lib;

use clap::{Arg,App};
use std::collections::HashMap;

// Project internal moduels
use advent_lib::utils;

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
    let quest = utils::read_input(filename);

    println!("Number of yes/all_yes answers: {:?}", process_program(quest));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base() {
        let doc = utils::read_input("../2020/unittest/day6-test-input.txt");
        assert_eq!(process_program(doc.clone()), (11,6));
    }
}

fn process_program(answers:Vec<String>) -> (u32,u32) {

    let mut yes_answer:u32 = 0;
    let mut all_yes_answers:u32 = 0;
    let mut persons:u32 = 0;
    let mut group:HashMap<char,u32> = HashMap::new();

    for answer in answers {
        if answer.is_empty() {
            yes_answer += group.len() as u32;
            for a in group {
                if a.1 == persons {
                    all_yes_answers += 1;
                }
            }
            persons = 0;
            group = HashMap::new();
            continue;
        }
        persons += 1;
        for y in answer.chars() {
            let entry = group.entry(y).or_insert(0);
            *entry += 1;
        }
    }
    // Last entry
    yes_answer += group.len() as u32;
    for a in group {
        if a.1 == persons {
            all_yes_answers += 1;
        }
    }

    return (yes_answer, all_yes_answers);
}