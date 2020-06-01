extern crate slack;


mod zeta;

use self::slack::RtmClient;


fn ping(channel: &str, cli: &RtmClient) {
    let _ = cli.sender().send_message(channel, "Failsafe is here!");
}

fn failsafe(channel: &str, cli: &RtmClient) {
    // TODO: stuff
    zeta::send_script();
    let (success, message) = zeta::try_start();
    if !success {
        let err = format!("[✗] Failed to activate failsafe, {}", &message);
        println!("{}", &err);
        let _ = cli.sender().send_message(channel, &err);
    } else {
        let success = format!(
            "[✓] Successfully ensured zeta is alive!: {}", &message);
        println!("{}", &success);
        let _ = cli.sender().send_message(channel, &success);
    }
}

pub fn respond(bot_id: &str, text: &str, channel: &str, cli: &RtmClient) {
    let ping_pattern = format!("<@{}> {}", bot_id, "failping");
    let failsafe_pattern = format!("<@{}> {}", bot_id, "failsafe");

    if text.contains(&ping_pattern) {
        ping(channel, cli)
    }
    else if text.contains(&failsafe_pattern) {
        failsafe(channel, cli)
    }
}
