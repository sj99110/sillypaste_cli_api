use serde_json::{json, Value};
use serde::{Serialize, Deserialize};
use hyper::{Body, Method, Client, Request};
use tokio;


#[derive(Serialize, Deserialize)]
pub struct PostData {
    author: Option<u32>,
    title: String,
    body: String,
    language: Option<u32>
    //expiry: Option<String>
}

pub fn build_paste(contents: String, author: Option<u32>, title: String, expiry: Option<String>) -> PostData {
    let data: PostData = PostData {
        author: author,
        title: title,
        body: contents,
        language: Some(470)
        //expiry: None
    };
    return data;
}


