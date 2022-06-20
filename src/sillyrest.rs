mod user;
mod post;

use hyper::{Body, Client, Request, Response, Method};
use hyper::client::connect::{HttpConnector};
use hyper_tls;
use hyper_tls::{HttpsConnector};
use serde_json::{json, Value};
use tokio;

pub struct SillyPasteClient {
    user: Option<user::User>,
    token: Option<String>,
    connection: Client<HttpsConnector<HttpConnector>>,
    uri: String
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
            user: Some(user),
            token: Some(token),
            connection: conn,
            uri: uri
        });
    }
    pub async fn upload_paste(&self, contents: String, title: String, expiry: Option<u32>) -> Result<(), String>{
        let conn = self.connection.clone();
        let uri = self.uri.clone() + "/api/paste/";
        println!("{}", uri.clone());
        let data = match &self.user {
            Some(c) => {
                let token = self.token.clone().unwrap();
                //let author = self.uri.clone() + "/api/user/" + &c.id().to_string() + "/";
                let body = json!(post::build_paste(contents, Some(c.id()), title, None));
                println!("{}", body.clone().to_string());
                Request::builder().
                    method(Method::POST).
                    uri(uri).
                    header("content-type", "application/json").
                    header("Authoirzation", String::from("Token ") + &token).
                    body(Body::from(body.to_string())).
                    expect("auth post error")
                },
            None => {
                let body = json!(post::build_paste(contents, None, title, None));
                println!("{}", body.clone().to_string());
                Request::builder().
                    method(Method::POST).
                    header("content-type", "application/json").
                    uri(uri).
                    body(Body::from(body.to_string())).
                    expect("upload_paste failed")
            }
        } ;
        let resp = match conn.request(data).await {
            Ok(c) => c,
            Err(_) => return Err(String::from("paste failed"))
        };
        if !resp.status().is_success() {
            let (parts, body) = resp.into_parts();
            println!("{:#?}\n {:#?}", parts, hyper::body::to_bytes(body).await.unwrap());
            return Err(String::from("postfail"));
        }
        return Ok(());
    }
}
