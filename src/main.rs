mod utils;
mod bot;

use crate::utils::read_from_toml;
use crate::bot::Bot;
use clap::{App, Arg, Subcommand};

fn main() {
    let config = read_from_toml(".env");
    let protocol = config.get("PROTOCOL").unwrap();
    let base_uri = config.get("BASE_URI").unwrap();
    let token = config.get("ACCESS_TOKEN").unwrap();
    let shared_secret = config.get("SHARED_SECRET").unwrap();
    let bot = Bot::new(protocol, base_uri, token, shared_secret);
    let matches = App::new("matrixbot")
        .version("1.0")
        .author("Lorenzo Carbonell <a.k.a. atareao>")
        .about("A cerca de")
        .arg(Arg::new("debug")
             .short('d')
             .long("debug")
             .takes_value(false))
        .subcommand(App::new("message")
                    .about("Send message")
                    .arg(Arg::new("room")
                         .short('r')
                         .required(true)
                         .takes_value(true))
                    .arg(Arg::new("text")
                         .short('t')
                         .required(true)
                         .takes_value(true))
                    .arg(Arg::new("markdown")
                         .short('m')
                         .takes_value(false)))
        .get_matches();
    if let Some(matches) = matches.subcommand_matches("message"){
        let room = matches.value_of("room").unwrap();
        let text = matches.value_of("text").unwrap();
        if matches.is_present("markdown"){
            bot.send_markdown_message(room, text)
        }else{
            bot.send_simple_message(room, text)
        }
    }
    /*
    let room = "!vWjVZOSPcAcQrJyqVG";
    let text = "Mensaje de prueba";
    bot.send_simple_message(room, text);
    bot.send_markdown_message(room, "Esto es **negrita** y esto *cursiva* con **markdown**");
    match bot.create_user("pepito", "pepito", false){
        Ok(response) => println!("Ok: {}", response.text().unwrap()),
        Err(response) => println!("Error: {}", response.to_string()),
    }
    if bot.is_username_vailable("pepito"){
        println!("Pepito está disponible")
    }else{
        println!("Pepito no está disponible")
    }
    */
}
