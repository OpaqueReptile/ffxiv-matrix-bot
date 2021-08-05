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

extern crate chrono;
use chrono::TimeZone;
use chrono::prelude::*;
use chrono::Duration;


//xivapi

pub(crate) fn countdown(bot: &ActiveBot, message: &Message, cmd: &str) -> HandleResult {
    let room = &message.room;
    let mut msg:String;
    //let cmd_split = cmd.split_whitespace();
    //for target_str in cmd_split {
        //match target_str.parse::<u32>() {
            //Ok(target_epoch) => {
                //let target_dt = Utc.timestamp(target_epoch, 0);
                let target_dt: DateTime<Utc> = Utc.ymd(2021, 11, 19).and_hms(8, 0, 0);
                let now_dt: DateTime<Utc> = Utc::now();
                let time_left = target_dt - now_dt;
                if time_left.num_weeks() > 3 {
                    msg = format!("There are {} weeks until Endwalker begins early access, kupo!", time_left.num_weeks());
                } else if time_left.num_days() > 3 {
                    msg = format!("There are {} days until Endwalker begins early access, kupo!!", time_left.num_days());
                } else if time_left.num_hours() > 1 {
                    msg = format!("There are {} hours until Endwalker begins early access, kupo!!!", time_left.num_hours());
                } else if time_left.num_minutes() > 1 {
                    msg = format!("There are {} minutes until Endwalker begins early access, kupo!!! 🌕👀", time_left.num_minutes());
                } else if time_left.num_seconds() > 1 {
                    msg = format!("Only {} seconds to go until Endwalker, kupo!!!!!!! 🌕👀", time_left.num_seconds());
                } else {
                    msg = format!("Enwalker is here, kupo! 🎉");
                }
                bot.send_message(
                    &msg,
                    room,
                    MessageType::RoomNotice,
                );
           /** }
            Err(_) => {
                bot.send_message(
                    &format!("{} is not a valid unix epoch time, kupo!", dice),
                    room,
                    MessageType::RoomNotice,
                );
                return HandleResult::StopHandling;
            }
        };**/
   // }
    HandleResult::StopHandling
}
