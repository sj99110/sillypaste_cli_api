use serde_json::{json, Value};
use serde::{Serialize, Deserialize};
use hyper::{Body, Method, Client, Request};
use std::vec;
use tokio;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostData {
    id: Option<u32>, // read only. none on send
    author: Option<u32>,
    title: String,
    body: String,
    language: Option<u32>,
    expiry: Option<String>,
    timestamp: Option<String>,
    hits: Option<u32>,
    freeze_hits: Option<bool>,
    size: Option<u32>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostList {
    count: u32,
    next: Option<String>,
    previous: Option<String>,
    results: Vec<PostData>
}

impl PostList {
    pub fn count(&self) -> u32 {
        return self.count;
    }
    pub fn posts(&self) -> Vec<PostData> {
        return self.results.clone();
    }
}

impl PostData {
    pub fn new_upload(contents: String, author: u32, title: String, expiry: Option<String>) -> PostData {
        PostData {
            id: None,
            author: Some(author),
            title: title,
            body: contents,
            language: None,
            expiry: expiry,
            timestamp: None,
            hits: None,
            freeze_hits: None,
            size: None
        }
    }
    pub fn id(&self) -> Option<u32> {
        return self.id;
    }
    pub fn author(&self) -> Option<u32> {
        return self.author;
    }
    pub fn title(&self) -> String {
        return self.title.clone();
    }
    pub fn body(&self) -> String {
        return self.body.clone();
    }
    pub fn language(&self) -> Option<u32> {
        return self.language;
    }
    pub fn expiry(&self) -> Option<String> {
        return self.expiry.clone();
    }
    pub fn timestamp(&self) -> Option<String> {
        return self.timestamp.clone();
    }
    pub fn hits(&self) -> Option<u32> {
        return self.hits;
    }
    pub fn freeze_hits(&self) -> Option<bool> {
        return self.freeze_hits;
    }
    pub fn size(&self) -> Option<u32> {
        return self.size;
    }
}

pub fn build_paste(contents: String, author: u32, title: String, expiry: Option<String>) -> PostData {
    return PostData::new_upload(contents, author, title, expiry);
}

pub fn parse_post_list(list: hyper::body::Bytes) -> Result<PostList, String> {
    //let posts : PostList = serde_json::from_slice(&list).unwrap();
    //println!("{:#?}", posts);
    match serde_json::from_slice(&list) {
        Ok(c) => return Ok(c),
        Err(_) => return Err(String::from("unable to retrieve post list"))
    }
    return Err(String::from("??"));
}
