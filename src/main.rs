mod utils;
mod bot;

use crate::utils::read_from_toml;
use crate::bot::Bot;

fn main() {
    let config = read_from_toml(".env");
    let protocol = config.get("PROTOCOL").unwrap();
    let base_uri = config.get("BASE_URI").unwrap();
    let token = config.get("ACCESS_TOKEN").unwrap();
    let shared_secret = config.get("SHARED_SECRET").unwrap();
    let bot = Bot::new(protocol, base_uri, token, shared_secret);
    let room = "!ayQGacclEOtNNfEicP";
    let text = "Mensaje de prueba";
    bot.send_simple_message(room, text);
    bot.send_makrdown_message(room, "Esto es **negrita** y esto *cursiva*");
    match bot.request_nonce(){
        Ok(result) => println!("{}", result),
        Err(result) => println!("{}", result),
    }
}
