
extern crate regex;
extern crate advent_lib;

use regex::Regex;

// Project internal moduels
use advent_lib::utils;

#[derive(Clone)]
struct Program {
    token: String,
    operation: bool,
    value: i32
}

fn main() {
    // Load the input file
    let datafile = advent_lib::utils::data_dir("2020", "day8-input.txt");
    let acc = utils::read_input(&datafile);

    println!("Accumulator: {:?}", process_program(acc));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let doc = utils::read_input(
            &advent_lib::utils::unittest_dir("2020", "day8-test-input.txt")
        );
        assert_eq!(process_program(doc), (5, 8));
    }
}


fn execute_program(instructions: &mut Vec<Program>) -> ( i32, i32 ) {
    let mut pos :usize = 0;
    let mut exec:Vec<usize> = Vec::new();
    let mut faulty_acc :i32 = 0;
    let mut acc :i32 = 0;
    let mut has_faulty: bool = false;
    

    loop {
        if pos >= instructions.len() { break }

        let mut cur = &instructions[pos];
        match cur.token.as_str() {
            "acc" => {
                if cur.operation { acc += cur.value } else { acc -= cur.value }
                pos += 1;                
            },
            "nop" => {
                pos += 1
            },
            "jmp" => {

                if cur.operation { pos += cur.value as usize } else { pos -= cur.value as usize }
            }
            _ => println!("Invalid token")
        }
        //println!("{:?} - {:?} {:?} {:?}", pos, cur.token, cur.operation, cur.value);

        if exec.contains(&pos) { 
            if ! has_faulty { 
                faulty_acc = acc;
                has_faulty = true;
            }
            
            // backwards loop
            let mut reach = false;
            loop {
                pos = exec.pop().unwrap();
                cur = &instructions[pos];

                if cur.token == "jmp" {
                    if reach { 
                        pos += 1;
                        break 
                    } else { 
                        reach = true;
                        continue
                    }
                } else if cur.token == "nop" { 
                    if reach { 
                        if cur.operation { pos -= cur.value as usize } else { pos += cur.value as usize }
                        break 
                    } else { 
                        reach = true ;
                        continue
                    }
                }
                if cur.token == "acc" {
                    if cur.operation { acc -= cur.value } else { acc += cur.value }
                }
            }
            
            continue;
        }

        // Add last executed instriction
        exec.push(pos);
    }

    (faulty_acc, acc)
}

fn process_program(program:Vec<String>) -> (i32, i32) {
    let mut instructions:Vec<Program> = Vec::new();   
    let re = Regex::new(r"^([a-z]{3}) ([+-])(\d+)").unwrap();

    for entry in &program {
        let cap = re.captures(&entry).unwrap();
        let data = Program {
            token: cap.get(1).unwrap().as_str().to_string(),
            operation: if cap.get(2).unwrap().as_str().parse::<char>().unwrap() == '+' { true } else { false },
            value: cap.get(3).unwrap().as_str().parse::<i32>().unwrap()
        };

        instructions.push( data );
    }

    execute_program(instructions.as_mut())
}
