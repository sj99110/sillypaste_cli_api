mod user;
mod post;

use hyper::{Body, Client, Request, Response, Method};
use hyper::client::connect::{HttpConnector};
use hyper_tls;
use hyper_tls::{HttpsConnector};
use serde_json::{json, Value};
use serde::{Serialize, Deserialize};
use std::vec;
use tokio;
use std::collections::{BTreeMap};

#[derive(Serialize, Deserialize, Clone)]
pub struct LanguageDE {
    pub name: String,
    pub id: u32
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LangList {
    count: u32,
    next: Option<String>,
    prev: Option<String>,
    results: Vec<LanguageDE>
}

type SillyError = (String, u32);

pub struct SillyPasteClient {
    user: user::User,
    token: String,
    connection: Client<HttpsConnector<HttpConnector>>,
    uri: String
}

impl LangList {
    pub fn into_map(&self) -> BTreeMap<String, u32> {
        let mut lang_map = BTreeMap::new();
        let list_iter = self.results.clone().into_iter();
        for x in list_iter {
            lang_map.insert(x.name, x.id);
        }
        return lang_map;
    }
}

impl SillyPasteClient {
    pub async fn new(username: String, password: String, uri: String) -> Result<SillyPasteClient, String> {
        let tls_conn = hyper_tls::HttpsConnector::new();
        let conn = Client::builder().
            build::<_, Body>(tls_conn);
        let token = match user::login(username, password, conn.clone(), uri.clone()).await {
            Ok(c) => c,
            Err(_) => return Err(String::from("login failed"))
        };
        let user = match user::get_user_info(token.clone(), conn.clone(), uri.clone()).await {
            Ok(c) => c,
            Err(_) => return Err(String::from("login failed. unable to get user info"))
        };
        return Ok(SillyPasteClient {
            user: user,
            token: token,
            connection: conn,
            uri: uri
        });
    }
    async fn send_request(&self, uri: String, method: Method, body: String) -> Result<Body, SillyError> {
        let data = Request::builder().
            method(method).
            uri(uri).
            header("content-type", "application/json").
            header("Authoirzation", String::from("Token ") + &self.token).
            body(Body::from(body)).
            expect("");
        let resp = match self.connection.request(data).await {
            Ok(c) => c,
            Err(_) => return Err((String::from("failed to get response"), 0))
        };
        if !resp.status().is_success() {
            return Err((String::from(""), resp.status().as_u16() as u32));
        }
        let (parts, body) = resp.into_parts();
        return Ok(body);
    }
    pub async fn upload_paste(&self, contents: String, title: String, expiry: Option<String>) -> Result<(), SillyError>{
        let uri = self.uri.clone() + "/api/paste/";
        let body = json!(post::build_paste(contents, self.user.id(), title, None));
        match self.send_request(uri, Method::POST, body.to_string()).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        };
        return Ok(());
    }
    pub async fn fetch_posts(&self, limit: u32, offset: u32) -> Result<Vec<post::PostData>, SillyError> {
        let uri = self.uri.clone() + "/api/paste/?limit=" + &limit.to_string() +
            "?offset=" + &(offset * 25).to_string();
        let body = match self.send_request(uri, Method::GET, String::from("")).await {
            Ok(b) => b,
            Err(e) => return Err(e)
        };
        let posts = match post::parse_post_list(hyper::body::to_bytes(body).await.unwrap()) {
            Ok(c) => return Ok(c.posts()),
            Err(_) => return Err((String::from("fetch fail"), 1))
        };
    }
    pub async fn retrieve_post(&self, post_id: u32) -> Result<post::PostData, SillyError> {
        let uri = self.uri.clone() + "/api/paste/" + &post_id.to_string() + "/";
        let data = match self.send_request(uri, Method::GET, String::from("")).await {
            Ok(c) => c,
            Err(e) => return Err(e)
        };
        let body = match hyper::body::to_bytes(data).await {
            Ok(c) => c,
            Err(_) => return Err((String::from("to bytes failure in retrieve posts"), 0))
        };
        println!("{:#?}", &body);
        return match serde_json::from_slice(&body) {
            Ok(c) => Ok(c),
            Err(e) =>  {
                println!("serde {:#?}", e);
                return Err((String::from("error parsing post data"), 0));
            }
        }
    }
    pub async fn retrieve_language_codes(&self) -> BTreeMap<String, u32> {
        let uri = self.uri.clone() + "/api/language/?limit=500";
        let data = self.send_request(uri, Method::GET, String::from("")).await.unwrap();
        let lang_list: LangList = serde_json::from_slice(&hyper::body::to_bytes(data).await.unwrap()).unwrap();
        return lang_list.into_map();
    }
}
