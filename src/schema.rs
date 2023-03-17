use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct RepliesData {
    children: Vec<String>
}

#[derive(Debug, Serialize, Deserialize)]
struct Replies {
    kind: String,
    data: RepliesData
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum RepliesEnum {
    Replies(Replies),
    String(String)
}

#[derive(Debug, Serialize, Deserialize)]
struct Comment {
    id: String,
    author: String,
    body: String,
    body_html: String,
    replies: RepliesEnum,
    score: usize
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Submission {
    id: String,
    author: String,
    comments: Vec<Comment>,
    domain: String,
    ups: usize,
    downs: usize,
    name: String,
    num_comments: usize,
    score: usize,
    selftext: String,
    selftext_html: String,
    subreddit: String,
    subreddit_id: String,
    title: String,
    url: String
}