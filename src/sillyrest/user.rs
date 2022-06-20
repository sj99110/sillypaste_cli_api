use serde_json::{json, Value};
use serde::{Serialize, Deserialize};
use hyper::{Body, Method, Client, Request};
use hyper::client::connect::{HttpConnector};
use hyper_tls::{HttpsConnector};
use std::string;
use std::str;

#[derive(Serialize, Deserialize, Clone)]
pub struct User{
    id: u32,
    is_staff: bool,
    username: String
}

impl User {
    pub fn id(&self) -> u32 {
        return self.id;
    }
    
    pub fn is_staff(&self) -> bool {
        return self.is_staff;
    }
    
    pub fn username(&self) -> String {
        return self.username.clone();
    }
}

pub async fn login(username: String, password: String, conn: Client<HttpsConnector<HttpConnector>>, uri: String) -> Result<String, String> {
    let uri = uri + "/api/login/";
    let post = Request::builder().
        method(Method::POST).
        uri(uri).
        header("content-type", "application/json").
        body(Body::from((json!({
            "username": username,
            "password": password})).to_string())).
        expect("login error");
    let resp = match conn.request(post).await {
        Ok(c) => c,
        Err(_) => return Err(String::from("authentication error"))
    };
    if !resp.status().is_success() {
        return Err(resp.status().to_string());
    }
    let (_, body) = resp.into_parts();
    let json: Value = serde_json::from_slice(&hyper::body::to_bytes(body).await.unwrap()).unwrap();
    let token = json["token"].as_str().unwrap().to_string();
    return Ok(token);
}

pub async fn logout(token: String, conn: Client<HttpsConnector<HttpConnector>>, uri: String) -> ()
{
    let uri = uri + "/api/login/";
    let request = Request::builder().
        method(Method::POST).
        uri(uri).
        header("Authorization", String::from("Token ") + &token).
        header("content-type", "application/json").
        body(Body::from("")).
        expect("logout");
        conn.request(request).await.unwrap();
        return ()
    }

pub async fn get_user_info(token: String, conn: Client<HttpsConnector<HttpConnector>>, uri: String) -> Result<User, String> {
    let uri = uri + "/api/user/me/";
    let request = Request::builder().
        method(Method::GET).
        uri(uri).
        header("Authorization", String::from("Token ") + &token).
        header("content-type", "application/json").
        body(Body::from("")).
        expect("get user info");
    let resp = match conn.request(request).await {
        Ok(c) => c,
        Err(_) => return Err(String::from("failed to get user info. user.rs 48"))
    };
    if !resp.status().is_success() {
        return Err(String::from("token rejected"));
    }
    let (_, body) = resp.into_parts();
    let user: User = serde_json::from_slice(&hyper::body::to_bytes(body).await.unwrap()).unwrap();
    return Ok(user);
}
    
