mod utils;

use std::collections::HashMap;
use crate::utils::read_from_toml;

fn main() {
    let config: HashMap<String, String> = read_from_toml(".env");
    let token = config.get("TOKEN").unwrap();
    let id = config.get("ID").unwrap();
    println!("Token: {}, Id: {}", token, id);
}
