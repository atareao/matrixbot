mod utils;
mod bot;

use std::collections::HashMap;
use crate::utils::read_from_toml;
use crate::bot::Bot;

fn main() {
    let config: HashMap<String, String> = read_from_toml(".env");
    let token = config.get("ACCESS_TOKEN").unwrap();
    let url = config.get("URL").unwrap();
    println!("Access token: {}, Url: {}", token, url);
    let bot = Bot::new(url, token);
    let room = "!ayQGacclEOtNNfEicP:synapse.territoriolinux.es";
    let text = "Mensaje de prueba";
    bot.send_simple_message(room, text)
}
