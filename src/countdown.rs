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
use chrono::prelude::*;
use chrono::Duration;
use chrono::TimeZone;

use std::thread;

//xivapi

fn countdown_message(target_dt: &DateTime<Utc>) -> String {
    let msg: String;
    let now_dt: DateTime<Utc> = Utc::now();
    let time_left = *target_dt - now_dt;
    if time_left.num_weeks() > 5 {
        msg = format!(
            "There are {} weeks until Endwalker early access, kupo!",
            time_left.num_weeks()
        );
    } else if time_left.num_days() > 2 {
        msg = format!(
            "There are {} days until Endwalker early access, kupo!!",
            time_left.num_days()
        );
    } else if time_left.num_hours() > 1 {
        msg = format!(
            "There are {} hours until Endwalker early access, kupo!!!",
            time_left.num_hours()
        );
    } else if time_left.num_minutes() > 1 {
        msg = format!(
            "There are {} minutes until Endwalker early access, kupo!!! 🌕👀",
            time_left.num_minutes()
        );
    } else if time_left.num_seconds() > 1 {
        msg = format!(
            "Only {} seconds to go until Endwalker, kupo!!!!!!! 🌕👀",
            time_left.num_seconds()
        );
    } else {
        msg = format!("Endwalker is here, kupo! 🎉");
    }
    msg
}

fn detailed_countdown_message(target_dt: &DateTime<Utc>) -> String {
    let msg: String;
    let now_dt: DateTime<Utc> = Utc::now();
    let time_left = *target_dt - now_dt;
    let weeks_left = Duration::weeks(time_left.num_weeks());
    let days_left = Duration::days(time_left.num_days()) - weeks_left;
    let hours_left = Duration::hours(time_left.num_hours()) - weeks_left - days_left;
    let minutes_left =
        Duration::minutes(time_left.num_minutes()) - weeks_left - days_left - hours_left;
    let seconds_left = Duration::seconds(time_left.num_seconds())
        - weeks_left
        - days_left
        - hours_left
        - minutes_left;
    let milliseconds_left = Duration::milliseconds(time_left.num_milliseconds())
        - weeks_left
        - days_left
        - hours_left
        - minutes_left
        - seconds_left;
    let microseconds_left = Duration::microseconds(time_left.num_microseconds().unwrap())
        - weeks_left
        - days_left
        - hours_left
        - minutes_left
        - seconds_left
        - milliseconds_left;
    let nanoseconds_left = Duration::nanoseconds(time_left.num_nanoseconds().unwrap())
        - weeks_left
        - days_left
        - hours_left
        - minutes_left
        - seconds_left
        - milliseconds_left
        - microseconds_left;
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
        msg = format!(
            "Only {} nanoseconds to go until blastoff, kupo!!!!!!! 🚀",
            time_left.num_nanoseconds().unwrap()
        );
    } else {
        msg = format!("Endwalker is here, kupo! 🎉🚀");
    }
    msg
}

#[allow(unreachable_code)]
pub(crate) fn countdown_loop(bot: &ActiveBot, message: &Message, cmd: &str) -> HandleResult {
    let room = &message.room;
    let target_dt: DateTime<Utc> = Utc.ymd(2021, 12, 3).and_hms(9, 0, 0);
    let mut t0 = String::from("");
    loop {
        let now = countdown_message(&target_dt).clone();
        if t0.as_str().ne(now.as_str()) {
            bot.send_message(
                &countdown_message(&target_dt),
                room,
                MessageType::RoomNotice,
            );
            t0 = countdown_message(&target_dt).clone();
        }
        println!("{}", countdown_message(&target_dt).as_str());
        thread::sleep(std::time::Duration::from_millis(15_000)); // sleep 15s
    }
    HandleResult::StopHandling
}

pub(crate) fn countdown(bot: &ActiveBot, message: &Message, cmd: &str) -> HandleResult {
    let room = &message.room;
    let target_dt: DateTime<Utc> = Utc.ymd(2021, 12, 3).and_hms(9, 0, 0);
    let msg: String;
    let now_dt: DateTime<Utc> = Utc::now();
    let time_left = target_dt - now_dt;
    if cmd
        .to_string()
        .trim_start_matches(" ")
        .trim_end_matches(" ")
        .len()
        == 0
    {
        msg = detailed_countdown_message(&target_dt);
    } else if cmd.to_string().contains("week") {
        msg = format!(
            "Only {} weeks to go until blastoff, kupo! 🚀",
            time_left.num_weeks()
        );
    } else if cmd.to_string().contains("day") {
        msg = format!(
            "Only {} days to go until blastoff, kupo! 🚀",
            time_left.num_days()
        );
    } else if cmd.to_string().contains("hour") {
        msg = format!(
            "Only {} hours to go until blastoff, kupo! 🚀",
            time_left.num_hours()
        );
    } else if cmd.to_string().contains("min") {
        msg = format!(
            "Only {} minutes to go until blastoff, kupo! 🚀",
            time_left.num_minutes()
        );
    } else if cmd.to_string().contains("mil") {
        msg = format!(
            "Only {} milliseconds to go until blastoff, kupo! 🚀",
            time_left.num_milliseconds()
        );
    } else if cmd.to_string().contains("mic") {
        msg = format!(
            "Only {} microseconds to go until blastoff, kupo! 🚀",
            time_left.num_microseconds().unwrap()
        );
    } else if cmd.to_string().contains("nan") {
        msg = format!(
            "Only {} nanoseconds to go until blastoff, kupo! 🚀",
            time_left.num_nanoseconds().unwrap()
        );
    } else if cmd.to_string().contains("sec") {
        msg = format!(
            "Only {} seconds to go until blastoff, kupo! 🚀",
            time_left.num_seconds()
        );
    } else {
        msg = format!(
            "Not sure what the heck you mean by \"{}\", kupo!",
            cmd.to_string()
                .trim_start_matches(" ")
                .trim_end_matches(" ")
        );
    }
    bot.send_message(&msg, room, MessageType::RoomNotice);
    HandleResult::StopHandling
}
