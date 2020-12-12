
extern crate regex;

extern crate advent_lib;

use regex::Regex;
use std::collections::HashMap;

// Project internal moduels
use advent_lib::utils;

fn main() {
    // Load the input file
    let datafile = advent_lib::utils::data_dir("2020", "day7-input.txt");
    let bags = utils::read_input(&datafile);

    println!("The bags that fit Shiny: {:?}", process_program(bags));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let doc = utils::read_input(
            &advent_lib::utils::unittest_dir("2020", "day7-test-input.txt")
        );
        assert_eq!(process_program(doc), (4, 32));
    }
    #[test]
    fn test_02() {
        let doc = utils::read_input(
            &advent_lib::utils::unittest_dir("2020", "day7-test-input2.txt")
        );
        assert_eq!(process_program(doc), (0, 126));
    }
}

fn fit_shiny_gold(bag_color: &String, bag_collection: &HashMap<String, HashMap<String,u32>> ) -> bool {
    let content = bag_collection.get(bag_color).unwrap();
    
    if bag_color == "shiny gold" { return false }
    if content.contains_key("shiny gold") { return true }

    for c in content {
        if fit_shiny_gold(c.0, &bag_collection) { return true }
       }
    return false;
}

fn count_shiny_gold(bag_color: &String, bag_collection: &HashMap<String, HashMap<String,u32>> ) -> u32 {
    let content = bag_collection.get(bag_color).unwrap();

    let mut amount :u32 = 0;
    for c in content {     
        amount += c.1 + ( c.1 * count_shiny_gold(c.0, &bag_collection));
    }
    
    return amount;
}

fn process_program(bag_colors:Vec<String>) -> (u32,u32) {
    let mut bags:HashMap<String, HashMap<String,u32>> = HashMap::new();
    let re = Regex::new(r"^(.*?) bags contain (.?*)\.").unwrap();
    let re_content = Regex::new(r"(\d) (.*?) bag").unwrap();
    
    // Assembly the bag color collections
    for entry in bag_colors {
        let mut bag_content:HashMap<String,u32> = HashMap::new();
        let cap = re.captures(&entry).unwrap();

        // Get main bag type
        let bag = cap.get(2).unwrap().as_str();
        for capture in re_content.captures_iter(bag) {
            bag_content.insert(
                capture.get(2).unwrap().as_str().to_string(),
                capture.get(1).unwrap().as_str().parse::<u32>().unwrap());
        }
        bags.insert(
            cap.get(1).unwrap().as_str().to_string(),
            bag_content
        );
    }

    let mut fit:u32 = 0;
    for c in &bags {
        if fit_shiny_gold(c.0, &bags) {
            fit += 1;
        }
    }

    let count = count_shiny_gold( &"shiny gold".to_string(), &bags);

    return ( fit, count );
}