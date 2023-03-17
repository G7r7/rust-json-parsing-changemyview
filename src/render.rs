use std::{path::Path, fs::File, io::Write};

use crate::schema::Submission;

pub fn render(value: &Submission, path: &Path) {
    let mut file: File;
    
    match File::create(path) {
        Ok(value) => file = value,
        Err(err) => panic!("Can't open file at {}: {err}", path.to_string_lossy())
    }
    
    write!(file, "<div>").unwrap();
    write!(file, "<div class=\"post\">").unwrap();
    write!(file, "<h3>{}</h3>", value.author).unwrap();
    write!(file, "<h1>{}</h1>", value.title).unwrap();
    write!(file, "<p>{}</p>", value.selftext).unwrap();
    write!(file, "</div>").unwrap();
    write!(file, "</div>").unwrap();
}