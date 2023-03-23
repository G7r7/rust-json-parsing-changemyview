use std::{path::Path, fs::File, io::Write};

use crate::schema::Submission;

pub fn render(value: &Submission, path: &Path) {
    let mut file: File;
    
    match File::create(path) {
        Ok(value) => file = value,
        Err(err) => panic!("Can't open file at {}: {err}", path.to_string_lossy())
    }
    
    write!(file, 
        "<style>
        .post{{border:3px solid grey; }}
        .comment{{border:3px solid turquoise; }}
        </style>"
    ).unwrap();
    write!(file, "<div>").unwrap();
    write!(file, "<div class=\"post\">").unwrap();
    write!(file, "<h3>{}</h3>", value.author).unwrap();
    write!(file, "<h1>{}</h1>", value.title).unwrap();
    write!(file, "<p>{}</p>", value.selftext).unwrap();
    let mut sorted_comments = value.comments.clone();
    let _ = &sorted_comments.sort_by_key(|comment| -comment.score);
    for comment in &sorted_comments {
        write!(file, "<div class=\"comment\">").unwrap();
        write!(file, "<h3>{}</h3>", comment.author).unwrap();
        write!(file, "<h3>Score : {}</h3>", comment.score).unwrap();
        write!(file, "<p>{}</p>", comment.body).unwrap();
        write!(file, "</div>").unwrap();
    }
    write!(file, "</div>").unwrap();
    write!(file, "</div>").unwrap();
}