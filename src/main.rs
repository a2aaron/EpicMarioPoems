extern crate egg_mode;
extern crate dotenv;
extern crate fs2;
extern crate rand;
extern crate tokio_core;

mod name_gen;
mod epic;

use dotenv::dotenv;
use std::env;
use name_gen::*;
use egg_mode::tweet::DraftTweet;
use tokio_core::reactor::Core;

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



    println!("{}", name_gen::random_msg());
}