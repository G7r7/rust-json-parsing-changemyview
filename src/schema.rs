use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RepliesData {
    pub children: Vec<String>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Replies {
    pub kind: String,
    pub data: RepliesData
}


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum RepliesEnum {
    Replies(Replies),
    String(String)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Comment {
    pub id: String,
    pub author: String,
    pub body: String,
    pub body_html: String,
    pub replies: RepliesEnum,
    pub score: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Submission {
    pub id: String,
    pub author: String,
    pub comments: Vec<Comment>,
    pub domain: String,
    pub ups: usize,
    pub downs: usize,
    pub name: String,
    pub num_comments: usize,
    pub score: i32,
    pub selftext: String,
    pub selftext_html: String,
    pub subreddit: String,
    pub subreddit_id: String,
    pub title: String,
    pub url: String
}