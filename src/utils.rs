use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, HeaderName};

pub fn post(url: &str, headers: &HashMap<String, String>)->Result<String, String>{
    let mut header_map = HeaderMap::new();
    for keyvalue in headers{
        header_map.insert(HeaderName::from_str(keyvalue.0).unwrap(),
                          HeaderValue::from_str(keyvalue.1).unwrap());
    }
    let client = Client::builder()
        .default_headers(header_map)
        .build()
        .unwrap();
    let result = client.post(url).send();
    match result{
        Ok(response) => Ok(response.text().unwrap()),
        Err(status) => Err(status.to_string()),
    }
}

pub fn read_from_toml(filename: &str)->HashMap<String, String>{
    let mut options:HashMap<String, String> = HashMap::new();
    let file = File::open(filename).unwrap();
    let lines = BufReader::new(file).lines();
    for line in lines {
        let keyvalue = line.unwrap();
        let v: Vec<&str> = keyvalue.split('=').collect();
        let key = v[0].trim().to_string();
        let value = v[1].trim().to_string();
        options.insert(key, value);
    }
    options
}

pub fn read_from_file(filename: &str) -> String {
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

