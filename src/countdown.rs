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

use std::thread;


//xivapi

fn countdown_message() -> String {
    let msg:String;
    let target_dt: DateTime<Utc> = Utc.ymd(2021, 11, 19).and_hms(9, 0, 0);
    let now_dt: DateTime<Utc> = Utc::now();
    let time_left = target_dt - now_dt;
    if time_left.num_weeks() > 5 {
        msg = format!("There are {} weeks until Endwalker early access, kupo!", time_left.num_weeks());
    } else if time_left.num_days() > 2 {
        msg = format!("There are {} days until Endwalker early access, kupo!!", time_left.num_days());
    } else if time_left.num_hours() > 1 {
        msg = format!("There are {} hours until Endwalker early access, kupo!!!", time_left.num_hours());
    } else if time_left.num_minutes() > 1 {
        msg = format!("There are {} minutes until Endwalker early access, kupo!!! 🌕👀", time_left.num_minutes());
    } else if time_left.num_seconds() > 1 {
        msg = format!("Only {} seconds to go until Endwalker, kupo!!!!!!! 🌕👀", time_left.num_seconds());
    } else {
        msg = format!("Endwalker is here, kupo! 🎉");
    }
    msg
}

fn detailed_countdown_message() -> String {
    let msg:String;
    let target_dt: DateTime<Utc> = Utc.ymd(2021, 11, 19).and_hms(9, 0, 0);
    let now_dt: DateTime<Utc> = Utc::now();
    let time_left = target_dt - now_dt;
    let weeks_left = Duration::weeks(time_left.num_weeks());
    let days_left = Duration::days(time_left.num_days()) - weeks_left;
    let hours_left = Duration::hours(time_left.num_hours()) - weeks_left - days_left;
    let minutes_left = Duration::minutes(time_left.num_minutes()) - weeks_left - days_left - hours_left;
    let seconds_left = Duration::seconds(time_left.num_seconds()) - weeks_left - days_left - hours_left - minutes_left;
    let milliseconds_left = Duration::milliseconds(time_left.num_milliseconds()) - weeks_left - days_left - hours_left - minutes_lef - seconds_left;
    let microseconds_left = Duration::microseconds(time_left.num_microseconds().unwrap()) - weeks_left - days_left - hours_left - minutes_lef - seconds_left - milliseconds_left;
    let nanoseconds_left = Duration::nanoseconds(time_left.num_nanoseconds().unwrap()) - weeks_left - days_left - hours_left - minutes_lef - seconds_left - milliseconds_left - microseconds_left;
    if time_left.num_weeks() > 0 {
        msg = format!("There are {} weeks, {} days, {} hours, {} minutes, {} seconds, {} milliseconds, {} microseconds, and {} nanoseconds until blastoff, kupo! 🚀",
                      weeks_left.num_weeks(), days_left.num_days(), hours_left.num_hours(), minutes_left.num_minutes(), seconds_left.num_seconds(), milliseconds_left.num_milliseconds(), microseconds_left.num_microseconds().unwrap(), nanoseconds_left.num_nanoseconds().unwrap());
    } else if time_left.num_days() > 0 {
        msg = format!("There are {} days, {} hours, {} minutes, {} seconds, {} milliseconds, {} microseconds, and {} nanoseconds until blastoff, kupo!! 🚀",
                      days_left.num_days(), hours_left.num_hours(), minutes_left.num_minutes(), seconds_left.num_seconds(), milliseconds_left.num_milliseconds(), microseconds_left.num_microseconds().unwrap(), nanoseconds_left.num_nanoseconds().unwrap());
    } else if time_left.num_hours() > 0 {
        msg = format!("There are {} hours, {} minutes, {} seconds, {} milliseconds, {} microseconds, and {} nanoseconds until blastoff, kupo!!! 🚀",
                      hours_left.num_hours(), minutes_left.num_minutes(), seconds_left.num_seconds(), milliseconds_left.num_milliseconds(), microseconds_left.num_microseconds().unwrap(), nanoseconds_left.num_nanoseconds().unwrap());
    } else if time_left.num_minutes() > 0 {
        msg = format!("There are {} minutes, {} seconds, {} milliseconds, {} microseconds, and {} nanoseconds until blastoff, kupo!!! 🚀",
                      minutes_left.num_minutes(), seconds_left.num_seconds(), milliseconds_left.num_milliseconds(), microseconds_left.num_microseconds().unwrap(), nanoseconds_left.num_nanoseconds().unwrap());
    } else if time_left.num_seconds() > 0 {
        msg = format!("Only {} nanoseconds to go until blastoff, kupo!!!!!!! 🚀", time_left.num_nanoseconds().unwrap());
    } else {
        msg = format!("Endwalker is here, kupo! 🎉🚀");
    }
    msg
}

pub(crate) fn countdown_loop(bot: &ActiveBot, message: &Message, cmd: &str) -> HandleResult {
    let room = &message.room;
    let mut t0 = String::from("");
    loop {
        let now = countdown_message().clone();
        if t0.as_str().ne(now.as_str()) {
            bot.send_message(
                &countdown_message(),
                room,
                MessageType::RoomNotice,
            );
            t0 = countdown_message().clone();
        }
        println!("{}",countdown_message().as_str());
        thread::sleep(std::time::Duration::from_millis(15_000)); // sleep 15s
    }
    HandleResult::StopHandling
}


pub(crate) fn countdown(bot: &ActiveBot, message: &Message, cmd: &str) -> HandleResult {
    let room = &message.room;
    //let mut msg:String;
    //let cmd_split = cmd.split_whitespace();
    //for target_str in cmd_split {
        //match target_str.parse::<u32>() {
            //Ok(target_epoch) => {
                //let target_dt = Utc.timestamp(target_epoch, 0);

                bot.send_message(
                    &detailed_countdown_message(),
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


