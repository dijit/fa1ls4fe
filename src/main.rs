extern crate failsafe_slack_bot;
extern crate slack;

use failsafe_slack_bot::handler;
use slack::RtmClient;
use std::{env, process};

fn main() {
    println!(
"   ___     _ _ __        __      
  / __\\_ _(_) / _\\ __ _ / _| ___ 
 / _\\/ _` | | \\ \\ / _` | |_ / _ \\
/ / | (_| | | |\\ \\ (_| |  _|  __/
\\/   \\__,_|_|_\\__/\\__,_|_|  \\___|
 Zeta Integrity Protection System
"
);
    let api_key: String = api_key();
    let mut handler = handler::Handler;
    let r = RtmClient::login_and_run(&api_key, &mut handler);

    match r {
        Ok(_) => {},
        Err(err) => panic!("Error: {}", err)
    }
}

fn api_key() -> String {
    match env::var("SLACK_API_TOKEN") {
        Ok(val) => val,
        Err(_) => {
            println!("Required the SLACK_API_TOKEN environment variable");
            process::exit(1);
        }
    }
}
