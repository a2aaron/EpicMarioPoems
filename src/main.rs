extern crate egg_mode;
extern crate dotenv;

mod name_gen;
mod epic;

use dotenv::dotenv;
use std::env;
use name_gen::*;



fn main() {
    dotenv().ok();

    let consumer_key = env::var("CONSUMER_KEY").unwrap();
    let consumer_secret = env::var("CONSUMER_SECRET").unwrap();
    let access_token = env::var("ACCESS_TOKEN").unwrap();
    let access_token_secret = env::var("ACCESS_TOKEN_SECRET").unwrap();

    let con_token = egg_mode::KeyPair::new(consumer_key, consumer_secret);
    let access_token = egg_mode::KeyPair::new(access_token, access_token_secret);
    let token = egg_mode::Token::Access {
        consumer: con_token,
        access: access_token,
    };



    for epic in epic::EPICS {
        match epic.wiki() {
            Some(wiki) => {
                let title = name_gen::super_mario_epic(epic.title);
                println!("{} ({})", title, wiki);
            }
            None => {
                println!(
                    "{}",
                    name_gen::super_mario_epic(epic.title),
                )
            }
        }
    }
}