use serde_json;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::path::Path;
use std::env;

mod line_count;
use line_count::{count_lines, get_line_content};
mod render;
use render::render;
mod schema;
use schema::Submission;

fn main() {
    // Reading and checking command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Invalid number of arguments.")
    }
    let file_path = &args[1];

    // Openning file and counting lines
    let path = Path::new(file_path);
    let line_nb = count_lines(&path);
    if line_nb <= 0 {
        panic!("Invalid number of lines.")
    }
    println!("{line_nb} lines counted in the file: {file_path}");

    // Parse all submissions
    let mut check_syntax = false;
    loop {
        println!("Check syntax (default: n)? (y/n):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<String>() {
            Ok(value) => {if value == "y" {check_syntax = true; break} else if value == "n" || value.len() == 0 {break}},
            Err(_) => continue,
        }
    }
    if check_syntax {
        let mut checked_line_nb = 0;
        let mut invalid_line_nb = 0;
        let reader = BufReader::new(File::open(path).unwrap());
        let iterator = reader.lines();
        iterator.flatten().for_each(
            |string| { 
                checked_line_nb += 1;
                match serde_json::from_str::<Submission>(&string) {
                    Ok(_) => { println!("Checked {checked_line_nb}/{line_nb}") },
                    Err(err) => {invalid_line_nb += 1; println!("Error parsing line n° {checked_line_nb}: {err}");},
                };
            }
        );
        println!("Checking done: {{line_nb - invalid_line_nb}} valids and {invalid_line_nb} invalids.")
    }


    // Reading user input for line number
    let mut selected_line_nb: usize = 0;
    while selected_line_nb <= 0 || selected_line_nb > line_nb {
        println!("Type in a valid line number to analyze (1-{line_nb}): ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse() {
            Ok(value) => selected_line_nb = value,
            Err(_) => continue,
        }
    }

    // Parsing the selected line
    let string = get_line_content(&path, selected_line_nb);
    // dbg!(string);
    let json: Submission;
    match serde_json::from_str(&string) {
        Ok(value) => json = value,
        Err(err) => panic!("Can't parse line content: {err}"),
    }

    // Asking for render path
    const DEFAULT_PATH: &str = "./render.html";
    println!("Type in render file path (default: {DEFAULT_PATH}): ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input = input.trim().into();
    if input.len() == 0 { input = DEFAULT_PATH.to_string() }
    let out_path = Path::new(&input);

    render(&json, out_path);
}
