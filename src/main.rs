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
        .subcommand(App::new("create")
                    .about("Create room, user (to get more info set object)")
                    .subcommand(App::new("user")
                                .about("Create user")
                                .arg(Arg::new("username")
                                     .help("The user name")
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
                                     .takes_value(false))
                                )
                    .subcommand(App::new("room")
                                .about("Create room")
                                .arg(Arg::new("room_name")
                                     .help("The room name")
                                     .short('r')
                                     .required(true)
                                     .takes_value(true)
                                     )
                                )
                    )
        .subcommand(App::new("block")
                    .about("Block")
                    .subcommand(App::new("room")
                                .about("Block a room")
                                .arg(Arg::new("room")
                                     .short('r')
                                     .required(true)
                                     .takes_value(true))
                                )
                    )
        .subcommand(App::new("remove")
                    .about("Remove room, user (to get more info set object)")
                    .subcommand(App::new("user")
                                .about("Remove user")
                                )
                    .subcommand(App::new("room")
                                .about("Remove room")
                                )
                    )
        .subcommand(App::new("join")
                    .about("Join")
                    .subcommand(App::new("room")
                                .about("Join room")
                                .arg(Arg::new("room")
                                     .short('r')
                                     .required(true)
                                     .takes_value(true))
                                )
                    )
        .subcommand(App::new("clear")
                    .about("Clear")
                    .subcommand(App::new("room")
                                .about("Clear room")
                                .arg(Arg::new("room")
                                     .short('r')
                                     .required(true)
                                     .takes_value(true))
                                )
                    )
        .subcommand(App::new("send")
                    .about("Send")
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
                                     .takes_value(false))
                                )
                    .subcommand(App::new("image")
                                .about("Send image")
                                )
                    )
        .get_matches();
    if let Some(sub) = matches.subcommand_matches("create"){
        if let Some(subsub) = sub.subcommand_matches("user"){
            let username = subsub.value_of("username").unwrap();
            let password = subsub.value_of("password").unwrap();
            let admin = subsub.is_present("admin");
            match bot.create_user(username, password, admin){
                Ok(result) => println!("User created: {}", result.status()),
                Err(result) => println!("Can not create the user: {}", result),
            }
        }else if let Some(subsub) = sub.subcommand_matches("room"){
            let roomname = subsub.value_of("roomname").unwrap();
            match bot.create_room(roomname){
                Ok(result) => println!("Room created: {}", result.status()),
                Err(result) => println!("Can not create the room: {}", result),
            }
        }
    }else if let Some(sub) = matches.subcommand_matches("send"){
        if let Some(subsub) = sub.subcommand_matches("message"){
            let room = subsub.value_of("room").unwrap();
            let text = subsub.value_of("text").unwrap();
            if subsub.is_present("markdown"){
                bot.send_markdown_message(room, text)
            }else{
                bot.send_simple_message(room, text)
            }
        }
    }else if let Some(sub) = matches.subcommand_matches("join"){
        if let Some(subsub) = sub.subcommand_matches("room"){
            let room = subsub.value_of("room").unwrap();
            bot.join_room(room)
        }
    }else if let Some(sub) = matches.subcommand_matches("clear"){
        if let Some(subsub) = sub.subcommand_matches("room"){
            let room = subsub.value_of("room").unwrap();
            bot.clear_room(room)
        }
    }else if let Some(sub) = matches.subcommand_matches("block"){
        if let Some(subsub) = sub.subcommand_matches("room"){
            let room = subsub.value_of("room").unwrap();
            bot.block_room(room)
        }
    }
}
