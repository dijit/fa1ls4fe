extern crate const_env;
extern crate failsafe_slack_bot;
extern crate slack;

use self::const_env::from_env;

use failsafe_slack_bot::handler;
use slack::RtmClient;
use std::{env, process};

#[from_env("SLACK_API_TOKEN")]
const SLACK_API_TOKEN: &'static str = "placeholder_token";

fn main() {
    println!(
        "   ___     _ _ __        __      
  / __\\_ _(_) / _\\ __ _ / _| ___ 
 / _\\/ _` | | \\ \\ / _` | |_ / _ \\
/ / | (_| | | |\\ \\ (_| |  _|  __/
\\/Zeta Integrity Protection System
 
"
    );
    let mut help_output: bool = false;
    let args: Vec<String> = env::args().collect();
    for arg in args {
        if arg.contains(&"--help") {
            help_output = true;
        } else if arg.contains(&"-h") {
            help_output = true;
        }
    }
    if help_output {
        println!("☶ Protocol 1: link to bot");
        println!("☵ Protocol 2: uphold the mission");
        println!("☳ Protocol 3: protect the bot");
        println!("                              ℐℋ");
        process::exit(0);
    }
    let api_key: String = SLACK_API_TOKEN.to_owned();
    let mut handler = handler::Handler;
    loop {
        //FIXME: crash here every time we lose websocket connection
        let r = RtmClient::login_and_run(&api_key, &mut handler);

        match r {
            Ok(_) => {}
            Err(err) => eprintln!("<!> Initiating restart sequence due to a {}", err),
        }
    }
}
