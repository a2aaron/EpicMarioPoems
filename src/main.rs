extern crate egg_mode;
extern crate dotenv;
extern crate fs2;
extern crate rand;
extern crate tokio_core;

mod name_gen;
mod epic;

use std::env;

use dotenv::dotenv;
use egg_mode::tweet::DraftTweet;
use tokio_core::reactor::Core;

use name_gen::random_msg;

fn main() {
    let token = get_token();

    if token.is_err() {
        println!("Failed to find a .env file: {:?}", token);
    } else {
        let epic = random_msg();
        let tweet = DraftTweet::new(&epic);
        
        let mut core = Core::new().unwrap();
        let handle = core.handle();
        let result = core.run(tweet.send(&token.unwrap(), &handle));
        match result {
            Ok(_) => println!("Successfully posted tweet: {}", epic),
            Err(err) => println!("Couldn't post tweet, reason: {}", err),
        }
    }
}

fn get_token() -> Result<egg_mode::Token, TokenError> {
    dotenv().ok();

    let consumer_key = env::var("CONSUMER_KEY");
    let consumer_secret = env::var("CONSUMER_SECRET");
    let access_token = env::var("ACCESS_TOKEN");
    let access_token_secret = env::var("ACCESS_TOKEN_SECRET");

    if consumer_key.is_err() || consumer_secret.is_err() ||
       access_token.is_err() || access_token_secret.is_err() {
        return Err(TokenError {
            consumer_key: consumer_key.err(),
            consumer_secret: consumer_secret.err(),
            access_token: access_token.err(),
            access_token_secret: access_token_secret.err(),
        })
    }

    let con_token = egg_mode::KeyPair::new(consumer_key.unwrap(), consumer_secret.unwrap());
    let access_token = egg_mode::KeyPair::new(access_token.unwrap(), access_token_secret.unwrap());

    Ok(egg_mode::Token::Access {
        consumer: con_token,
        access: access_token,
    })
}

#[derive(Debug)]
struct TokenError {
    consumer_key: Option<std::env::VarError>,
    consumer_secret: Option<std::env::VarError>,
    access_token: Option<std::env::VarError>,
    access_token_secret: Option<std::env::VarError>,
}

