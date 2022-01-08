use std::collections::HashMap;
use std::str::FromStr;
use reqwest::blocking::{Client, Response};
use reqwest::header::{HeaderMap, HeaderValue, HeaderName};
use reqwest::Error;
use sha1::Sha1;
use hmac::{Hmac, Mac};
use serde_json::Value;
use regex::Regex;

type HmacSha1 = Hmac<Sha1>;

trait Json{
    fn to_string(&self) -> String;
}

impl Json for HashMap<&str, &str>{
    fn to_string(&self) -> String{
        let mut items: Vec<String> = Vec::new();
        for (key, value) in self.iter(){
            items.push(format!("\"{}\": \"{}\"", key, value));
        }
        let result = vec!["{".to_string(), items.join(","), "}".to_string()];
        result.concat()
    }
}

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
        let mut body: HashMap<&str, &str> = HashMap::new();
        body.insert("msgtype", "m.text");
        body.insert("body", text);
        let result = post(&url, &headers, body.to_string());
        match result{
            Ok(response) => println!("OK: {}", response.text().unwrap()),
            Err(response) => println!("Err: {}", response.to_string()),
        }
    }
    pub fn send_makrdown_message(&self, room: &str, text: &str){
        let url = format!("{}://{}/_matrix/client/r0/rooms/{}:{}/send/m.room.message",
               self.protocol, self.base_uri, room, self.base_uri);
        println!("URL: {}", url);
        let mut headers: HashMap<String, String> = HashMap::new();
        headers.insert("Authorization".to_string(), format!("Bearer {}", self.token));
        let mut body: HashMap<&str, &str> = HashMap::new();
        let mut html = markdown::to_html(text);
        //html = parse_special_chars(&html[..html.len()-1]);
        html = html[..html.len()-1].to_string();
        println!("{}", &html);
        body.insert("msgtype", "m.text");
        body.insert("format", "org.matrix.custom.html");
        body.insert("body", text);
        body.insert("formatted_body", &html);
        println!("Resultado: {}", body.to_string());
        let result = post(&url, &headers, body.to_string());
        match result{
            Ok(response) => println!("OK: {}", response.text().unwrap()),
            Err(response) => println!("Err: {}", response.to_string()),
        }

    }
    pub fn request_nonce(&self)->Result<String, String>{
        let url = format!("{}://{}/_synapse/admin/v1/register",
               self.protocol, self.base_uri);
        let mut headers: HashMap<String, String> = HashMap::new();
        headers.insert("Authorization".to_string(), format!("Bearer {}", self.token));
        match get(&url, &headers) {
            Ok(response) => {
                let v: Value = serde_json::from_str(&response.text().unwrap()).unwrap();
                Ok(remove_external_quotes(&v["nonce"].to_string()))
            },
            Err(result) => Err(result.to_string())
        }
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

pub fn post(url: &str, headers: &HashMap<String, String>, body: String)->Result<Response, Error>{
    let mut header_map = HeaderMap::new();
    for keyvalue in headers{
        header_map.insert(HeaderName::from_str(keyvalue.0).unwrap(),
                          HeaderValue::from_str(keyvalue.1).unwrap());
    }
    let client = Client::builder()
        .default_headers(header_map)
        .build()
        .unwrap();
    client.post(url).body(body).send()
}

pub fn generate_mac(shared_secret: &str, nonce: &str, user: &str, password: &str, admin: Option<bool>, user_type: Option<&str>) -> String{
    let mut hasher = HmacSha1::new_from_slice(shared_secret.as_bytes()).unwrap();
    hasher.update(nonce.as_bytes());
    hasher.update(b"\x00");
    hasher.update(user.as_bytes());
    hasher.update(b"\x00");
    hasher.update(password.as_bytes());
    hasher.update(b"\x00");
    if let Some(_isadmin) = admin{
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
    let admin = None;
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

#[test]
fn test_parse_special_chars(){
    let ejemplo ="<p>";
    assert_eq!("\\u003cp\\u003e", parse_special_chars(ejemplo));
}
