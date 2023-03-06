use std::env;
use std::fs;
use serde::Deserializer;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let file_content = fs::read_to_string(file_path).expect("Should have been able to read the file");
    // Deserializer::deserialize_string(self, visitor);

    println!("{}", file_content);
}
