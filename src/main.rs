mod utils;
mod bot;

use crate::utils::read_from_toml;
use crate::bot::Bot;
use clap::{App, Arg, AppSettings};

const NAME: &str =env!("CARGO_PKG_NAME");
const DESCRIPTION: &str =env!("CARGO_PKG_DESCRIPTION");
const VERSION: &str =env!("CARGO_PKG_VERSION");
const AUTHORS: &str =env!("CARGO_PKG_AUTHORS");

fn main() {
    let config = read_from_toml(".env");
    let protocol = config.get("PROTOCOL").unwrap();
    let base_uri = config.get("BASE_URI").unwrap();
    let token = config.get("ACCESS_TOKEN").unwrap();
    let shared_secret = config.get("SHARED_SECRET").unwrap();
    let bot = Bot::new(protocol, base_uri, token, shared_secret);
    let matches = App::new(NAME)
        .version(VERSION)
        .author(AUTHORS)
        .about(DESCRIPTION)
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(Arg::new("debug")
             .short('d')
             .long("debug")
             .takes_value(false))
        .subcommand(App::new("create_user")
                    .about("Manage user")
                    .arg(Arg::new("username")
                         .help("The username")
                         .short('u')
                         .required(true)
                         .takes_value(true))
                    .arg(Arg::new("password")
                         .help("Password for the user")
                         .short('p')
                         .required(true)
                         .takes_value(true))
                    .arg(Arg::new("admin")
                         .help("if present set user as admin")
                         .short('a')
                         .required(false)
                         .takes_value(false)))
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
    }else if let Some(mathes) = matches.subcommand_matches("create_user"){
        let username = matches.value_of("username").unwrap();
        let password = matches.value_of("password").unwrap();
        let admin = mathes.is_present("admin");
        match bot.create_user(username, password, admin){
            Ok(result) => println!("User created: {}", result.status()),
            Err(result) => println!("Can not create the user: {}", result),
        }
    }
}
