use serde_json::{Value};
use std::io;
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
    let json: Submission;
    match serde_json::from_str(&string) {
        Ok(value) => json = value,
        Err(err) => panic!("Can't parse line content: {err}"),
    }

    render(json);
}
