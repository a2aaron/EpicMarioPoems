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
    let epic = random_msg();

    match token {
        Err(token) => println!("Failed to find environment variable: {:?}\n\n{}", token, epic),
        Ok(token) => {
            let tweet = DraftTweet::new(&epic);
            let mut core = Core::new().unwrap();
            let handle = core.handle();
            let result = core.run(tweet.send(&token, &handle));
            match result {
                Ok(_) => println!("Successfully posted tweet!\n\n{}", epic),
                Err(err) => println!("Authentication Token Missing: {}\n\n{}", err, epic),
            }
        }
    }
}

fn get_token() -> Result<egg_mode::Token, TokenError> {
    dotenv().ok();

    let consumer_key = env::var("MARIO_EPIC_CONSUMER_KEY")
        .map_err(TokenError::ConsumerKey)?;
    let consumer_secret = env::var("MARIO_EPIC_CONSUMER_SECRET")
        .map_err(TokenError::ConsumerSecret)?;
    let access_token = env::var("MARIO_EPIC_ACCESS_TOKEN")
        .map_err(TokenError::AccessToken)?;
    let access_token_secret = env::var("MARIO_EPIC_ACCESS_TOKEN_SECRET")
        .map_err(TokenError::AccessTokenSecret)?;

    let con_token = egg_mode::KeyPair::new(consumer_key, consumer_secret);
    let access_token = egg_mode::KeyPair::new(access_token, access_token_secret);

    Ok(egg_mode::Token::Access {
        consumer: con_token,
        access: access_token,
    })
}

use std::env::VarError;
#[derive(Debug)]
enum TokenError {
    ConsumerKey(VarError),
    ConsumerSecret(VarError),
    AccessToken(VarError),
    AccessTokenSecret(VarError),
}
