extern crate egg_mode;
extern crate dotenv;

mod name_gen;
mod epic;

use dotenv::dotenv;
use std::env;
use name_gen::*;



fn main() {
    dotenv().ok();

    let consumer_key = env::var("CONSUMER_KEY");
    let consumer_secret = env::var("CONSUMER_SECRET");
    let access_token = env::var("ACCESS_TOKEN");
    let access_token_secret = env::var("ACCESS_TOKEN_SECRET");

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