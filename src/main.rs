use std::io;
use std::{env, io::BufRead};
use std::fs::File;
use serde_json::{Value};

mod line_count;
use line_count::count_lines;

fn main() {

    // Reading and checking command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Invalid number of arguments.")
    }
    let file_path = &args[1];

    // Openning file and counting lines
    let file = File::open(&file_path).unwrap();
    let line_nb = count_lines(&file);
    if line_nb <= 0 { panic!("Invalid number of lines.") }
    println!("{line_nb} lines counted in the file: {file_path}");
    
    // Reading user input for line number
    let mut selected_line_nb = 0u32;
    while  selected_line_nb <= 0 || selected_line_nb > line_nb {
        println!("Type in a valid line number to analyze (1-{line_nb}): ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<u32>() {
            Ok(value) => selected_line_nb = value,
            Err(_) => continue
        }
    }


    // println!("You typed: {}", input.trim());

    // let mut json_strings = Vec::<String>::new();

    // let lines = std::io::BufReader::new(file).lines();

    // for line in lines {
    //     if let Ok(json_string) = line {
    //         json_strings.push(json_string)
    //     }
    // }

    // let mut jsons = Vec::<Value>::new();

    // for json_string in json_strings {
    //     let result = serde_json::from_str(&json_string);
    //     match result {
    //         Ok(value) => {
    //             jsons.push(value)    
    //         }
    //         Err(err) => {
    //             panic!("{}", err)
    //         }
    //     }
    // }

    // println!("{}", jsons[0]["domain"]);
}