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
    let api_key: String = api_key();
    let mut handler = handler::Handler;
    loop {
        //FIXME: crash here every time we lose websocket connection
        let r = RtmClient::login_and_run(&api_key, &mut handler);

        match r {
            Ok(_) => {}
            Err(err) => eprintln!(" ⚠️  Initiating restart sequence: due to a {}", err),
        }
    }
}

fn api_key() -> String {
    match env::var("SLACK_API_TOKEN") {
        Ok(val) => val,
        Err(_) => {
            println!("⚠️  ➡️ Requires the SLACK_API_TOKEN environment variable");
            process::exit(1);
        }
    }
}
