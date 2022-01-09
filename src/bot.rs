use std::collections::HashMap;
use std::str::FromStr;
use reqwest::blocking::{Client, Response};
use reqwest::header::{HeaderMap, HeaderValue, HeaderName};
use reqwest::Error;
use sha1::Sha1;
use hmac::{Hmac, Mac};
use serde_json::{Value, json};
use regex::Regex;

type HmacSha1 = Hmac<Sha1>;

pub struct Bot{
    protocol: String,
    base_uri: String,
    token: String,
    shared_secret: String,
}

impl Bot{
    pub fn new(protocol: &str, base_uri: &str, token: &str, shared_secret: &str) -> Bot{
        Self {
            protocol: protocol.to_string(),
            base_uri: base_uri.to_string(),
            token: token.to_string(),
            shared_secret: shared_secret.to_string(),
        }
    }
    pub fn send_simple_message(&self, room: &str, text: &str){
        let url = format!("{}://{}/_matrix/client/r0/rooms/{}:{}/send/m.room.message",
               self.protocol, self.base_uri, room, self.base_uri);
        println!("URL: {}", url);
        let mut headers: HashMap<String, String> = HashMap::new();
        headers.insert("Authorization".to_string(), format!("Bearer {}", self.token));
        let body = json!({
            "msgtype": "m.text",
            "body": text
        });
        let result = post(&url, &headers, Some(serde_json::to_string(&body).unwrap()));
        match result{
            Ok(response) => println!("OK: {}", response.text().unwrap()),
            Err(response) => println!("Err: {}", response.to_string()),
        }
    }
    pub fn join_room(&self, room: &str) -> Result<Response, Error>{
        let url = format!("{}://{}/_matrix/client/r0/rooms/{}:{}/join",
               self.protocol, self.base_uri, room, self.base_uri);
        let mut headers: HashMap<String, String> = HashMap::new();
        headers.insert("Authorization".to_string(), format!("Bearer {}", self.token));
        post(&url, &headers, None)
    }

    pub fn send_markdown_message(&self, room: &str, text: &str){
        let url = format!("{}://{}/_matrix/client/r0/rooms/{}:{}/send/m.room.message",
               self.protocol, self.base_uri, room, self.base_uri);
        println!("URL: {}", url);
        let mut headers: HashMap<String, String> = HashMap::new();
        headers.insert("Authorization".to_string(), format!("Bearer {}", self.token));
        let mut html = markdown::to_html(text);
        //html = parse_special_chars(&html[..html.len()-1]);
        html = html[..html.len()-1].to_string();
        println!("{}", &html);
        let body = json!({
            "msgtype": "m.text",
            "format": "org.matrix.custom.html",
            "body": text,
            "formatted_body": html
        });
        let result = post(&url, &headers, Some(serde_json::to_string(&body).unwrap()));
        match result{
            Ok(response) => println!("OK: {}", response.text().unwrap()),
            Err(response) => println!("Err: {}", response.to_string()),
        }

    }
    pub fn is_username_vailable(&self, username: &str) -> bool{
        let url = format!("{}://{}/_matrix/client/r0/available?username={}",
                          self.protocol, self.base_uri, username);
        let mut headers: HashMap<String, String> = HashMap::new();
        headers.insert("Authorization".to_string(), format!("Bearer {}", self.token));
        match get(&url, &headers){
            Ok(response) => response.status() == 200,
            _ => false,
        }
    }

    pub fn create_user(&self, username: &str, password: &str, admin: bool)->Result<Response, Error>{
        match self.request_nonce(){
            Ok(response) => {
                let url = format!("{}://{}/_synapse/admin/v1/register",
                                  self.protocol, self.base_uri);
                let v: Value = serde_json::from_str(&response.text().unwrap()).unwrap();
                let nonce = remove_external_quotes(&v["nonce"].to_string());
                println!("Nonce: {}", &nonce);
                let mac = generate_mac(&self.shared_secret, &nonce, username,
                                       password, admin, None);
                println!("mac: {}", &mac);
                let mut headers: HashMap<String, String> = HashMap::new();
                headers.insert("Authorization".to_string(), format!("Bearer {}", self.token));
                let body = json!({
                    "nonce": &nonce,
                    "username": username,
                    "password": password,
                    "admin": admin,
                    "mac": mac
                });
                println!("Body: {}", serde_json::to_string(&body).unwrap());
                post(&url, &headers, Some(serde_json::to_string(&body).unwrap()))
            },
            Err(result) => Err(result),
        }
    }

    fn request_nonce(&self)->Result<Response, Error>{
        let url = format!("{}://{}/_synapse/admin/v1/register",
               self.protocol, self.base_uri);
        let mut headers: HashMap<String, String> = HashMap::new();
        headers.insert("Authorization".to_string(), format!("Bearer {}", self.token));
        get(&url, &headers)
    }

}
fn get(url: &str, headers: &HashMap<String, String>)->Result<Response, Error>{
    let mut header_map = HeaderMap::new();
    for keyvalue in headers{
        header_map.insert(HeaderName::from_str(keyvalue.0).unwrap(),
                          HeaderValue::from_str(keyvalue.1).unwrap());
    }
    let client = Client::builder()
        .default_headers(header_map)
        .build()
        .unwrap();
    client.get(url).send()
}

pub fn post(url: &str, headers: &HashMap<String, String>, body: Option<String>)->Result<Response, Error>{
    let mut header_map = HeaderMap::new();
    for keyvalue in headers{
        header_map.insert(HeaderName::from_str(keyvalue.0).unwrap(),
                          HeaderValue::from_str(keyvalue.1).unwrap());
    }
    let client = Client::builder()
        .default_headers(header_map)
        .build()
        .unwrap();
    match body{
        Some(content) => client.post(url).body(content).send(),
        None => client.post(url).send(),
    }
}

pub fn generate_mac(shared_secret: &str, nonce: &str, user: &str, password: &str, admin: bool, user_type: Option<&str>) -> String{
    let mut hasher = HmacSha1::new_from_slice(shared_secret.as_bytes()).unwrap();
    hasher.update(nonce.as_bytes());
    hasher.update(b"\x00");
    hasher.update(user.as_bytes());
    hasher.update(b"\x00");
    hasher.update(password.as_bytes());
    hasher.update(b"\x00");
    if admin{
        hasher.update(b"admin");
    }else{
        hasher.update(b"notadmin");
    }
    if let Some(usertype) = user_type {
        hasher.update(b"\x00");
        hasher.update(usertype.as_bytes());
    }
    let result = hasher.finalize();
    format!("{:x}", result.into_bytes())
}

#[test]
fn test_generate_mac(){
    let shared_secret = "secreto";
    let nonce = "nonce";
    let user = "user";
    let password = "password";
    let admin = false;
    let user_type = None;
    let result = generate_mac(shared_secret, nonce, user, password, admin, user_type);
    assert_eq!(result, "3b56e9050e1a170f8805c57537f96a36c02e29f0");
}

fn remove_external_quotes(string: &str)->String{
    let pattern = Regex::new(r#"^"(.*)"$"#).unwrap();
    if let Some(captures) = pattern.captures(string){
        return captures[1].to_string();
    }
    string.to_string()
}

#[test]
fn test_remove_external_quotes(){
    let ejemplo = "\"ejemplo\"";
    assert_eq!("ejemplo", remove_external_quotes(ejemplo));

}
