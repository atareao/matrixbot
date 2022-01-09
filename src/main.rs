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
    let room = "!vWjVZOSPcAcQrJyqVG";
    let text = "Mensaje de prueba";
    //bot.send_simple_message(room, text);
    //bot.send_markdown_message(room, "Esto es **negrita** y esto *cursiva* con **markdown**");
    match bot.create_user("pepito", "pepito", false){
        Ok(response) => println!("Ok: {}", response.text().unwrap()),
        Err(response) => println!("Error: {}", response.to_string()),
    }
    if bot.is_username_vailable("pepito"){
        println!("Pepito está disponible")
    }else{
        println!("Pepito no está disponible")
    }
}
