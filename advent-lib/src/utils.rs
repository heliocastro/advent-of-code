use std::env;
use std::process;
use std::path::Path;

pub fn read_input(file_name: &str) -> Vec<String> {
    std::fs::read_to_string(file_name)
        .expect("file not found!")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn code_dir() -> String {
    match env::var("CODE_HOME") {
        Ok(val) => return val,
        Err(e) => {
            eprintln!("Error: {:?}", e);
            process::exit(0)
        }
    }
}

fn file_dir(year :&str, filename :&str, type_dir :&str) -> String {
    let path= Path::new( &code_dir() )
        .join("rust")
        .join("advent-of-code")
        .join(year)
        .join(type_dir)
        .join(filename);
    path.to_str().unwrap().to_string()
}

pub fn unittest_dir(year :&str, filename :&str) -> String {
    file_dir(year, filename, "unittest")
}

pub fn data_dir(year :&str, filename :&str) -> String {
    file_dir(year, filename, "data")
}