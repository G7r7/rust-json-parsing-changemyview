use std::{fs::File, io::BufReader, io::BufRead};


pub fn count_lines(file: &File) -> u32 {
    let iterator = BufReader::new(file).lines();
    let mut counter = 0u32;
    for _line in iterator {
        counter += 1;
    }
    counter
}