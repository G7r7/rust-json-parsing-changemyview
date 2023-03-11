use std::{env, io::BufRead};
use std::fs::File;
use serde_json::{Value};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Invalid number of arguments.")
    }

    let file = File::open(&args[1]).unwrap();

    let mut json_strings = Vec::<String>::new();

    let lines = std::io::BufReader::new(file).lines();

    for line in lines {
        if let Ok(json_string) = line {
            json_strings.push(json_string)
        }
    }

    let mut jsons = Vec::<Value>::new();

    for json_string in json_strings {
        let result = serde_json::from_str(&json_string);
        match result {
            Ok(value) => {
                jsons.push(value)    
            }
            Err(err) => {
                panic!("{}", err)
            }
        }
    }

    println!("{}", jsons[0]["domain"]);
}