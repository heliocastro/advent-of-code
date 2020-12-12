extern crate regex;

extern crate advent_lib;

use regex::Regex;

// Project internal moduels
use advent_lib::utils;

fn main() {
    // Load the input file
    let datafile = advent_lib::utils::data_dir("2020", "day5-input.txt");
    let seats = utils::read_input(&datafile);

    println!("Highest Seat ID: {:?}", process_program(seats));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base() {
        assert_eq!(process_program(vec!["FBFBBFFRLR".to_string()]), 357);
        assert_eq!(process_program(vec!["BFFFBBFRRR".to_string()]), 567);
        assert_eq!(process_program(vec!["FFFBBBFRRR".to_string()]), 119);
        assert_eq!(process_program(vec!["BBFFBBFRLL".to_string()]), 820);
    }
}

fn seat_partition(data: String, pos:usize) -> u32 {
    let mut low = 1;
    let mut high = pos;
    let mut mid = pos/2;
    let mut res = 0;
    
    for cur in data.chars() {
        if cur == 'F' || cur == 'L' {    
            high = mid;
            mid = low + ((high - low) / 2);
            res = low;
        } else if cur == 'B' || cur == 'R'{
            low = mid + 1;
            mid = low + ((high - low) / 2);
            res = high;
        }
    }
    res = res - 1;
    
    return res as u32;
}

fn process_program(seat_allocation:Vec<String>) -> u32 {
    let mut top_seat_id:u32 = 0;
    let mut all_seats :Vec<u32> = Vec::new();
    let re = Regex::new(r"([^RL]*)([^FB]*)").unwrap();
    
    for entry in seat_allocation {
        let cap = re.captures(&entry).unwrap();
        let row_entries =  cap.get(1).map_or("", |m| m.as_str() );
        let col_entries =  cap.get(2).map_or("", |m| m.as_str() );

        let row :u32 = seat_partition(row_entries.to_string(), 128);
        let col :u32 = seat_partition(col_entries.to_string(), 8);

        let seat_id = row * 8 + col;
        if top_seat_id < seat_id { 
            top_seat_id = seat_id;
        }
        all_seats.push(seat_id);
    }

    all_seats.sort();
    for n in *all_seats.first().unwrap()..*all_seats.last().unwrap() {
        if ! all_seats.contains(&n) {
            println!("Your seat ID: {:?}", n);
        }
    }

    return top_seat_id;
}