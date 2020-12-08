extern crate matrix_bot_api;
use matrix_bot_api::handlers::{HandleResult, Message};
use matrix_bot_api::{ActiveBot, MessageType};

extern crate fractal_matrix_api;

extern crate reqwest;

// Just used for loading the username, password and homeserverurl from a file.
extern crate config;
// Just used for rolling dice
extern crate rand;

extern crate serde;
extern crate serde_json;

//xivapi

pub(crate) fn roll_help(bot: &ActiveBot, message: &Message, _cmd: &str) -> HandleResult {
    let mut help = "Roll dice:\n".to_string();
    help += "!roll X [X ..]\n";
    help += "with\n";
    help += "X = some number. Thats the number of eyes your die will have.\n";
    help += "If multpile numbers are given, multiple dice are rolled. The result as a sum is displayed as well.\n";
    help += "\nExample: !roll 6 12 => Rolls 2 dice, one with 6, the other with 12 eyes.\n";
    bot.send_message(&help, &message.room, MessageType::RoomNotice);
    HandleResult::ContinueHandling /* There might be more handlers that implement "help" */
}

pub(crate) fn roll_dice(bot: &ActiveBot, message: &Message, cmd: &str) -> HandleResult {
    let room = &message.room;
    let cmd_split = cmd.split_whitespace();

    let mut results: Vec<u32> = vec![];
    for dice in cmd_split {
        let sides = match dice.parse::<u32>() {
            Ok(x) => x,
            Err(_) => {
                bot.send_message(
                    &format!("{} is not a number.", dice),
                    room,
                    MessageType::RoomNotice,
                );
                return HandleResult::StopHandling;
            }
        };
        results.push((rand::random::<u32>() % sides) + 1);
    }

    if results.len() == 0 {
        return roll_help(bot, message, cmd);
    }

    if results.len() == 1 {
        bot.send_message(&format!("{}", results[0]), room, MessageType::RoomNotice);
    } else {
        // make string from results:
        let str_res: Vec<String> = results.iter().map(|x| x.to_string()).collect();
        bot.send_message(
            &format!("{} = {}", str_res.join(" + "), results.iter().sum::<u32>()),
            room,
            MessageType::RoomNotice,
        );
    }

    HandleResult::StopHandling
}
