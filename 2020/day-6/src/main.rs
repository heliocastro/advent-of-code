extern crate clap;
extern crate advent_lib;

use std::collections::HashMap;

// Project internal moduels
use advent_lib::utils;

fn main() {
    let datafile = advent_lib::utils::data_dir("2020", "day6-input.txt");
    let quest = utils::read_input(&datafile);

    println!("Number of yes/all_yes answers: {:?}", process_program(quest));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base() {
        let doc = utils::read_input(
            &advent_lib::utils::unittest_dir("2020", "day6-test-input.txt")
        );
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