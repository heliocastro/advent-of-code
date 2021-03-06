extern crate clap;
extern crate colored;

use clap::{Arg, App};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::mem;
use colored::*;

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
        
    let reader = BufReader::new(input);

    let mut data:Vec<String> = Vec::new();
    for line in reader.lines() {
        data.push(line.unwrap());
    }
    
    let res = process_program(data[0].clone(), data[1].clone());
    println!("Distance:{:?} Steps:{:?}", res.0, res.1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        assert_eq!(process_program(
            String::from("R8,U5,L5,D3"),
            String::from("U7,R6,D4,L4")),
        (6, 30));
    }
    #[test]
    fn test_02() {
        assert_eq!(process_program(
            String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72"),
            String::from("U62,R66,U55,R34,D71,R55,D58,R83")),
        (159,610));
    }
    #[test]
    fn test_03() {
        assert_eq!(process_program(
            String::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"),
            String::from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")),
        (135,410));
    } 
}


fn process_program(data_a:String, data_b:String) -> ( i32, i32 ) {
    let mut points:Vec<(i32,i32)> = Vec::new();
    let wires: Vec<String> = vec![data_a, data_b];
    let mut minimal_distance = 0;
    let mut minimal_steps = 0;
    let mut wiresize: Vec<usize> = Vec::new();

    println!("{}", "Processing wire program ( Advent 2019 - Day 3 )".yellow().bold());

    for wire in wires {
        let path: Vec<String> = wire.split(',').map(|x| x.parse::<String>().unwrap()).collect();
        wiresize.push(path.len());
        points.push((0,0));
        for input in &path {
            let mut elem = input.clone();
            let direction = elem.remove(0);
            let steps: i32 = elem.parse::<i32>().unwrap();
            let (x, y) = points.last().copied().unwrap();
 
            match direction {
                'U' => {
                        points.push((x, y + steps));
                    },
                'R' => {
                        points.push((x + steps, y));
                    },
                'L' => {
                        points.push((x - steps, y));
                    },
                'D' => {
                        points.push((x, y - steps));
                    },
                _ => println!("Invalid direction"),
            }
        }
    }

    let mut steps_a = 0;
    for j in 1..=wiresize[0] {
        let ( mut amin, mut amax, a_fixed, a_axis ) = if points[j-1].0 == points[j].0 { 
            (points[j-1].1, points[j].1, points[j].0, true) 
        } else { 
            (points[j-1].0, points[j].0, points[j].1, false) 
        };
        if amax < amin { mem::swap(&mut amax, &mut amin) }
        println!("{} <=> {}", amax, amin);
        if amin < 0 {
            steps_a = steps_a + (amax + amin.abs());
        } else {
            steps_a = steps_a + (amax - amin);
        }
        
        let mut steps_b = 0;
        for k in wiresize[0] + 2..=wiresize[0] + wiresize[1] + 1 {
            let (mut min, mut max, b_fixed, b_axis) = if points[k-1].0 == points[k].0 {
                (points[k-1].1, points[k].1, points[k].0, true)
            } else {
                (points[k-1].0, points[k].0, points[k].1, false)
            };
            
            if max < min { mem::swap(&mut max, &mut min) }
            if min < 0 {
                steps_b = steps_b + (max + min.abs());
            } else {
                steps_b = steps_b + (max - min);
            }

            if a_axis != b_axis {
                if  a_fixed >= min
                    && a_fixed <= max
                    && b_fixed >= amin
                    && b_fixed <= amax
                    && a_fixed + b_fixed != 0 {
                    let sum = a_fixed.abs() + b_fixed.abs();
                    if minimal_distance == 0 || minimal_distance > sum {
                        minimal_distance = sum;
                    }
                    
                    if ! b_axis {
                        let ( a, b ) = (steps_a.abs() - (b_fixed - amin), steps_b - (a_fixed - min));
                        if minimal_steps == 0 || minimal_steps > a+b { minimal_steps = a+b }
                        // println!("a={:?} b={:?}, {}{:?}", a_fixed, b_fixed, "Res=".green(), a+b);
                    } else {
                        let (a, b) = (steps_a.abs() - (b_fixed - amin), steps_b - ( max - a_fixed));
                        if minimal_steps == 0 || minimal_steps > a+b { minimal_steps = a+b }
                        // println!("a={:?} b={:?}, {}{:?}", a_fixed, b_fixed, "Res=".green(), minimal_steps);
                    }
                }
            }
        }
    }

    return ( minimal_distance, minimal_steps );
}